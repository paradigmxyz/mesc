use crate::MescCliError;
use mesc::Endpoint;

pub fn print_endpoint_json(endpoint: Endpoint) {
    match serde_json::to_string(&endpoint) {
        Ok(as_str) => println!("{}", as_str),
        Err(_) => eprintln!("could not serialize endpoint"),
    }
}

pub fn print_endpoint_pretty(endpoint: Endpoint) {
    println!("Endpoint: {}", endpoint.name);
    println!("- url: {}", endpoint.url);
    println!(
        "- chain_id: {}",
        endpoint
            .chain_id
            .map_or("-".to_string(), |chain_id| chain_id.to_string())
    );
    println!("- metadata: {:?}", endpoint.endpoint_metadata);
}

pub(crate) fn print_endpoints(config: &mesc::RpcConfig, reveal: bool) -> Result<(), MescCliError> {
    if config.endpoints.is_empty() {
        println!("[none]")
    } else {
        let mut endpoints = Vec::new();
        let mut networks = Vec::new();
        let mut urls = Vec::new();
        let mut sorted_endpoints: Vec<_> = config.endpoints.values().collect();
        sorted_endpoints.sort_by(|e1, e2| {
            e1.chain_id
                .clone()
                .unwrap_or(mesc::ChainId::null_chain_id())
                .cmp(
                    &e2.chain_id
                        .clone()
                        .unwrap_or(mesc::ChainId::null_chain_id()),
                )
        });
        for endpoint in sorted_endpoints.into_iter() {
            endpoints.push(endpoint.name.clone());
            networks.push(endpoint.chain_id_string());
            if reveal {
                urls.push(endpoint.url.clone());
            } else {
                urls.push("*".repeat(8));
            }
        }
        let format = toolstr::TableFormat::default();
        let format = toolstr::TableFormat {
            // indent: 4,
            // column_delimiter: " . ".to_string(),
            // header_separator_delimiter: " . ".to_string(),
            ..format
        };
        let mut table = toolstr::Table::default();
        table.add_column("endpoint", endpoints)?;
        table.add_column("network", networks)?;
        table.add_column("url", urls)?;
        format.print(table)?;
    };

    Ok(())
}
