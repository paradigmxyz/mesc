use crate::metadata::EndpointMetadata;
use crate::{metadata, MescCliError, PingArgs};
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub(crate) fn all_ping_fields() -> Vec<String> {
    vec![
        "ip".to_string(),
        "location".to_string(),
        "latency".to_string(),
        "block".to_string(),
        "client".to_string(),
    ]
}

pub(crate) async fn ping_command(args: PingArgs) -> Result<(), MescCliError> {
    let mut tasks = FuturesUnordered::<JoinHandle<_>>::new();

    let fields = if args.fields.contains(&"all".to_string()) {
        all_ping_fields()
    } else if args.fields.is_empty() {
        vec!["latency".to_string(), "block".to_string()]
    } else {
        args.fields
    };

    // get endpoints
    let mut query = mesc::EndpointQuery::new();
    if let Some(network) = args.network {
        query = query.chain_id(network)?;
    }
    if let Some(name) = args.name {
        query = query.name(name)?;
    }
    if let Some(url) = args.url {
        query = query.url(url)?;
    }
    let endpoints = mesc::find_endpoints(query)?;
    let n_endpoints = endpoints.len();

    for endpoint in endpoints.iter() {
        let name = endpoint.name.clone();
        let url = endpoint.url.clone();
        let fields = fields.clone();
        let task: JoinHandle<(String, Result<EndpointMetadata, MescCliError>)> =
            tokio::spawn(async move {
                let result = metadata::get_node_metadata(url.clone(), &fields, args.timeout).await;
                (name, result)
            });
        tasks.push(task);
    }
    let mut names = vec![];
    let mut networks = vec![];
    let mut metadatas = vec![];
    let mut ips = vec![];
    let mut block_numbers = vec![];
    let mut failed_endpoints = vec![];
    let mut latencies: Vec<String> = vec![];
    let mut locations: Vec<Option<String>> = vec![];
    while let Some(result) = tasks.next().await {
        match result {
            Ok((name, Ok(metadata))) => {
                metadatas.push(metadata);

                for endpoint in endpoints.iter() {
                    if endpoint.name == name {
                        networks.push(
                            endpoint
                                .chain_id
                                .clone()
                                .map(|c| c.to_string())
                                .unwrap_or("-".to_string()),
                        );
                        continue;
                    }
                }

                names.push(name);
            }
            Ok((name, Err(_))) => failed_endpoints.push(name.to_string()),
            Err(e) => return Err(MescCliError::JoinError(e)),
        }
    }

    let mut node_clients = Vec::new();
    for metadata in metadatas.into_iter() {
        node_clients.push(metadata.node_client_version.unwrap_or("-".to_string()));
        ips.push(metadata.ip_address.unwrap_or("-".to_string()));
        block_numbers.push(
            metadata
                .current_block
                .map(|b| b.to_string())
                .unwrap_or("-".to_string()),
        );
        latencies.push(
            metadata
                .latency
                .map(|l| (l * 1000.0).to_string()[0..5].to_string())
                .unwrap_or("-".to_string()),
        );
        locations.push(metadata.location);
    }
    let mut table = toolstr::Table::default();

    table.add_column("endpoint", names)?;
    table.add_column("network", networks)?;

    for field in fields.iter() {
        match field.as_str() {
            "latency" => table.add_column("latency\n(ms)", latencies.clone())?,
            "ip" => table.add_column("ip", ips.clone())?,
            "block" => table.add_column("block", block_numbers.clone())?,
            "location" => table.add_column("location", locations.clone())?,
            "client" => table.add_column("node client", node_clients.clone())?,
            _ => {
                return Err(MescCliError::InvalidInput(format!(
                    "unknown field: {}",
                    field
                )))
            }
        }
    }
    let format = toolstr::TableFormat::default();
    format.print(table)?;

    println!();
    if failed_endpoints.is_empty() {
        println!("{} endpoints responded without error", n_endpoints);
    } else {
        println!(
            "failed collection for {} of {} endpoints: {}",
            failed_endpoints.len(),
            n_endpoints,
            failed_endpoints.join(", ")
        );
    };

    let field_set: HashSet<_> = fields.into_iter().collect();
    let all_field_set: HashSet<_> = all_ping_fields().into_iter().collect();
    let additional_fields: Vec<_> = all_field_set.difference(&field_set).collect();
    let additional_fields: Vec<_> = additional_fields.iter().map(|s| s.as_str()).collect();
    if !additional_fields.is_empty() {
        println!();
        println!(
            "additional fields avaiable: {}",
            additional_fields.join(", ")
        );
    };

    Ok(())
}
