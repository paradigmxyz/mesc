mod cli;
mod printing;
mod setup;
mod status;

use clap::Parser;
use cli::{Cli, Commands};
use printing::{print_endpoint_json, print_endpoint_pretty, print_endpoint_url};

fn main() {
    match Cli::parse().command {
        Commands::Setup(_args) => setup::run_setup(),
        Commands::Status(_args) => status::print_status(),
        Commands::Url(args) => url_command(args.query, args.profile.as_deref()),
        Commands::Json(args) => json_command(args.query, args.profile.as_deref()),
        Commands::Pretty(args) => pretty_command(args.query, args.profile.as_deref()),
        Commands::Find(args) => find_command(args.chain_id, args.name, args.url, args.metadata),
    }
}

fn url_command(query: Option<String>, profile: Option<&str>) {
    print_endpoint_url(resolve_endpoint(query, profile));
}

fn json_command(query: Option<String>, profile: Option<&str>) {
    print_endpoint_json(resolve_endpoint(query, profile));
}

fn pretty_command(query: Option<String>, profile: Option<&str>) {
    print_endpoint_pretty(resolve_endpoint(query, profile));
}

fn resolve_endpoint(
    query: Option<String>,
    profile: Option<&str>,
) -> Result<Option<mesc::Endpoint>, mesc::ConfigError> {
    match query {
        Some(query) => mesc::parse_user_query(&query, profile),
        None => mesc::get_default_endpoint(profile),
    }
}

fn find_command(
    _chain_id: Option<u64>,
    _name: Option<String>,
    _url: Option<String>,
    _metadata: Vec<String>,
) {
    todo!()
}
