use crate::metadata::EndpointMetadata;
use crate::{metadata, MescCliError, PingArgs};
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
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
    let config_data = mesc::load::load_config_data()?;
    let mut tasks = FuturesUnordered::<JoinHandle<_>>::new();

    let fields = if args.fields.contains(&"all".to_string()) {
        all_ping_fields()
    } else if args.fields.is_empty() {
        vec!["latency".to_string(), "block".to_string()]
    } else {
        args.fields
    };

    for endpoint in config_data.endpoints.into_values() {
        let url = endpoint.url.clone();
        let fields = fields.clone();
        let task: JoinHandle<(String, Result<EndpointMetadata, MescCliError>)> =
            tokio::spawn(async move {
                let result = metadata::get_node_metadata(url.clone(), &fields).await;
                (endpoint.name, result)
            });
        tasks.push(task);
    }
    let mut names = vec![];
    let mut metadatas = vec![];
    let mut ips = vec![];
    let mut block_numbers = vec![];
    let mut failed_endpoints = vec![];
    let mut latencies: Vec<String> = vec![];
    let mut locations: Vec<Option<String>> = vec![];
    while let Some(result) = tasks.next().await {
        match result {
            Ok((name, Ok(metadata))) => {
                names.push(name);
                metadatas.push(metadata);
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

    if fields.contains(&"latency".to_string()) {
        table.add_column("latency (ms)", latencies)?;
    };
    if fields.contains(&"ip".to_string()) {
        table.add_column("ip", ips)?;
    };
    if fields.contains(&"block".to_string()) {
        table.add_column("latest block", block_numbers)?;
    };
    if fields.contains(&"location".to_string()) {
        table.add_column("location", locations)?;
    };
    if fields.contains(&"client".to_string()) {
        table.add_column("node client", node_clients)?;
    };
    let format = toolstr::TableFormat::default();
    format.print(table)?;

    println!();
    if failed_endpoints.is_empty() {
        println!("all endpoints responded without error");
    } else {
        println!(
            "failed collection for {} endpoints: {}",
            failed_endpoints.len(),
            failed_endpoints.join(", ")
        );
    };
    Ok(())
}
