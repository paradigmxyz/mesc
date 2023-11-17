mod cli;
mod find;
mod ping;
mod printing;
mod setup;
mod status;

use clap::Parser;
use cli::{Cli, Commands};
use printing::{print_endpoint_json, print_endpoint_pretty};

fn main() {
    match Cli::parse().command {
        Commands::Setup(_args) => setup::run_setup(),
        Commands::Status(_args) => status::print_status(),
        Commands::Url(args) => url_command(args),
        Commands::Endpoint(args) => endpoint_command(args),
        Commands::Find(args) => find::find_command(args.chain_id, args.name, args.url, args.metadata),
        Commands::Ping(_args) => ping::ping_command(),
    }
}

fn url_command(args: cli::UrlArgs) {
    let endpoint = match args.query {
        Some(query) => mesc::parse_user_query(query.as_str(), args.profile.as_deref()),
        None => mesc::get_default_endpoint(args.profile.as_deref()),
    };
    match endpoint {
        Ok(Some(endpoint)) => println!("{}", endpoint.url),
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    }
}

fn endpoint_command(args: cli::EndpointArgs) {
    let endpoint = match args.query {
        Some(query) => mesc::parse_user_query(query.as_str(), args.profile.as_deref()),
        None => mesc::get_default_endpoint(args.profile.as_deref()),
    };
    match endpoint {
        Ok(Some(endpoint)) => if args.json {
            print_endpoint_json(endpoint);
        } else {
            print_endpoint_pretty(endpoint);
        },
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    }
}
