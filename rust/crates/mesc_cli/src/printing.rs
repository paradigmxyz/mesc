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
