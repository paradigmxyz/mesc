#![allow(dead_code)]

mod cli;
mod find;
mod metadata;
mod ping;
mod printing;
mod rpc;
mod setup;
mod status;
mod types;

use clap::Parser;
use cli::{Cli, Commands};
use printing::{print_endpoint_json, print_endpoint_pretty};
use types::*;

#[tokio::main]
async fn main() -> Result<(), MescCliError> {
    match Cli::parse().command {
        Commands::Setup(_args) => setup::run_setup(),
        Commands::Status(_args) => status::print_status(),
        Commands::Url(args) => url_command(args),
        Commands::Endpoint(args) => endpoint_command(args),
        Commands::Find(args) => {
            find::find_command(args.chain_id, args.name, args.url, args.metadata)
        }
        Commands::Ping(_args) => ping::ping_command().await,
    }
}

fn url_command(args: cli::UrlArgs) -> Result<(), MescCliError> {
    let endpoint = match args.query {
        Some(query) => mesc::parse_user_query(query.as_str(), args.profile.as_deref()),
        None => mesc::get_default_endpoint(args.profile.as_deref()),
    };
    match endpoint {
        Ok(Some(endpoint)) => println!("{}", endpoint.url),
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    };
    Ok(())
}

fn endpoint_command(args: cli::EndpointArgs) -> Result<(), MescCliError> {
    let endpoint = match args.query {
        Some(query) => mesc::parse_user_query(query.as_str(), args.profile.as_deref()),
        None => mesc::get_default_endpoint(args.profile.as_deref()),
    };
    match endpoint {
        Ok(Some(endpoint)) => {
            if args.json {
                print_endpoint_json(endpoint);
            } else {
                print_endpoint_pretty(endpoint);
            }
        }
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    };
    Ok(())
}
