use mesc::{ConfigError, Endpoint};

pub(crate) fn print_endpoint_url(endpoint: Result<Option<Endpoint>, ConfigError>) {
    match endpoint {
        Ok(Some(endpoint)) => println!("{}", endpoint.url),
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    }
}

pub fn print_endpoint_json(endpoint: Result<Option<Endpoint>, ConfigError>) {
    match endpoint {
        Ok(Some(endpoint)) => match serde_json::to_string(&endpoint) {
            Ok(as_str) => println!("{}", as_str),
            Err(_) => eprintln!("could not serialize endpoint"),
        },
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    }
}

pub fn print_endpoint_pretty(endpoint: Result<Option<Endpoint>, ConfigError>) {
    match endpoint {
        Ok(Some(endpoint)) => {
            println!("Endpoint: {}", endpoint.name);
            println!("- url: {}", endpoint.url);
            println!("- chain_id: {}", endpoint.chain_id);
            println!("- metadata: {:?}", endpoint.endpoint_metadata);
        },
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    }
}
