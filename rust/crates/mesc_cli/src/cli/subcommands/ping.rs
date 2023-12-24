use crate::{network, network::EndpointNetworkInfo, MescCliError, PingArgs};
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashSet;
use tokio::task::JoinHandle;
use toolstr::ColumnFormatShorthand;

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

    let fields = if args.fields.contains(&"all".to_string()) | (args.json & args.fields.is_empty())
    {
        all_ping_fields()
    } else if args.fields.is_empty() {
        vec!["latency".to_string(), "block".to_string()]
    } else {
        args.fields
    };

    // get endpoints
    let mut query = mesc::MultiEndpointQuery::new();
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
        let task: JoinHandle<(String, Result<EndpointNetworkInfo, MescCliError>)> =
            tokio::spawn(async move {
                let result =
                    network::get_node_network_info(url.clone(), &fields, args.timeout).await;
                (name, result)
            });
        tasks.push(task);
    }
    let mut names = vec![];
    let mut urls = vec![];
    let mut networks = vec![];
    let mut network_infos = vec![];
    let mut ips = vec![];
    let mut block_numbers = vec![];
    let mut failed_endpoints = vec![];
    let mut latencies: Vec<String> = vec![];
    let mut locations: Vec<Option<String>> = vec![];
    while let Some(result) = tasks.next().await {
        match result {
            Ok((name, Ok(network_info))) => {
                network_infos.push(network_info);

                for endpoint in endpoints.iter() {
                    if endpoint.name == name {
                        networks.push(
                            endpoint
                                .chain_id
                                .clone()
                                .map(|c| c.to_string())
                                .unwrap_or("-".to_string()),
                        );
                        urls.push(endpoint.url.clone());
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
    for network_info in network_infos.into_iter() {
        node_clients.push(network_info.node_client_version.unwrap_or("-".to_string()));
        ips.push(network_info.ip_address.unwrap_or("-".to_string()));
        block_numbers
            .push(network_info.current_block.map(|b| b.to_string()).unwrap_or("-".to_string()));
        latencies.push(
            network_info
                .latency
                .map(|l| (l * 1000.0).to_string()[0..5].to_string())
                .unwrap_or("-".to_string()),
        );
        locations.push(network_info.location);
    }

    if args.json {
        let entries =
            names
                .into_iter()
                .zip(urls)
                .zip(networks)
                .zip(ips)
                .zip(locations)
                .zip(latencies)
                .zip(node_clients)
                .zip(block_numbers)
                .map(|(((((((name, url), network), ip), location), latency), client), block)| {
                    Entry { name, url, network, ip, location, latency, client, block }
                })
                .collect::<Vec<Entry>>();
        println!("{}", serde_json::to_string_pretty(&entries)?);
        return Ok(());
    }

    let mut title_style = crate::metadata::get_theme_font_style("title")?;
    title_style.bold();
    let metavar_style = crate::metadata::get_theme_font_style("metavar")?;
    let mut description_style = crate::metadata::get_theme_font_style("description")?;
    description_style.bold();
    let option_style = crate::metadata::get_theme_font_style("option")?;
    let _content_style = crate::metadata::get_theme_font_style("content")?;
    let comment_style = crate::metadata::get_theme_font_style("comment")?;

    let mut format = toolstr::TableFormat::default()
        .border_font_style(comment_style.clone())
        .label_font_style(title_style.clone());
    let mut table = toolstr::Table::default();

    table.add_column("endpoint", names)?;
    format.add_column(ColumnFormatShorthand::new().name("endpoint").font_style(metavar_style));
    table.add_column("network", networks)?;
    format.add_column(
        ColumnFormatShorthand::new().name("network").font_style(description_style.clone()),
    );

    for field in fields.iter() {
        match field.as_str() {
            "latency" => {
                table.add_column("latency", latencies.clone())?;
                format.add_column(
                    ColumnFormatShorthand::new()
                        .name("latency")
                        .font_style(description_style.clone()),
                );
            }
            "ip" => {
                table.add_column("ip", ips.clone())?;
                format.add_column(
                    ColumnFormatShorthand::new().name("ip").font_style(option_style.clone()),
                );
            }
            "block" => {
                table.add_column("block", block_numbers.clone())?;
                format.add_column(
                    ColumnFormatShorthand::new()
                        .name("block")
                        .font_style(description_style.clone()),
                );
            }
            "location" => {
                table.add_column("location", locations.clone())?;
                format.add_column(
                    ColumnFormatShorthand::new().name("location").font_style(option_style.clone()),
                );
            }
            "client" => {
                table.add_column("node client", node_clients.clone())?;
                format.add_column(
                    ColumnFormatShorthand::new()
                        .name("node client")
                        .font_style(option_style.clone()),
                );
            }
            _ => return Err(MescCliError::InvalidInput(format!("unknown field: {}", field))),
        }
    }
    format.print(table)?;

    println!();
    if failed_endpoints.is_empty() {
        println!(
            "{}",
            comment_style.format(format!("{} endpoints responded without error", n_endpoints))
        );
    } else {
        let text = format!(
            "failed collection for {} of {} endpoints: {}",
            failed_endpoints.len(),
            n_endpoints,
            failed_endpoints.join(", ")
        );
        println!("{}", comment_style.format(text));
    };

    let field_set: HashSet<_> = fields.into_iter().collect();
    let all_field_set: HashSet<_> = all_ping_fields().into_iter().collect();
    let additional_fields: Vec<_> = all_field_set.difference(&field_set).collect();
    let additional_fields: Vec<_> = additional_fields.iter().map(|s| s.as_str()).collect();
    if !additional_fields.is_empty() {
        println!();
        let text = format!("additional fields avaiable: {}", additional_fields.join(", "));
        println!("{}", comment_style.format(text));
    };

    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Entry {
    name: String,
    url: String,
    network: String,
    ip: String,
    location: Option<String>,
    latency: String,
    client: String,
    block: String,
}
