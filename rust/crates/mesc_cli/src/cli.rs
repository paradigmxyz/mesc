use clap::{Parser, Subcommand};

/// Utility for creating and managing MESC RPC configurations
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

/// Define your subcommands as an enum
#[derive(Subcommand)]
pub enum Commands {
    /// Interactively create new configuration
    Setup(SetupArgs),
    /// Print status of configuration
    Status(StatusArgs),
    /// Print endpoint URL
    Url(UrlArgs),
    /// Print endpoint
    Endpoint(EndpointArgs),
    /// Search through list of configured endpoints
    Find(FindArgs),
    /// Ping endpoints and get their client versions
    Ping(PingArgs),
}

/// Arguments for the `setup` subcommand
#[derive(Parser)]
pub struct SetupArgs {
    /// Example argument
    #[clap(short, long)]
    pub path: Option<String>,
}

/// Arguments for the `status` subcommand
#[derive(Parser)]
pub struct StatusArgs {
}

/// Arguments for the `url` subcommand
#[derive(Parser)]
pub struct UrlArgs {
    /// query
    #[clap(short, long)]
    pub query: Option<String>,

    /// profile
    #[clap(short, long)]
    pub profile: Option<String>,
}

/// Arguments for the `json` subcommand
#[derive(Parser)]
pub struct EndpointArgs {
    /// query
    #[clap(short, long)]
    pub query: Option<String>,

    /// print as json
    #[clap(short, long)]
    pub json: bool,

    /// profile
    #[clap(short, long)]
    pub profile: Option<String>,
}

/// Arguments for the `find` subcommand
#[derive(Parser)]
pub struct FindArgs {
    /// chain id
    #[clap(short, long)]
    pub chain_id: Option<u64>,

    /// name (fuzzy match)
    #[clap(short, long)]
    pub name: Option<String>,

    /// url (fuzzy match)
    #[clap(short, long)]
    pub url: Option<String>,

    /// metadata, space-separated key=value pairs
    #[clap(short, long)]
    pub metadata: Vec<String>,
}

/// Arguments for the `ping` subcommand
#[derive(Parser)]
pub struct PingArgs {
    /// ping only endpoints of this network
    #[clap(short, long)]
    pub network: Option<String>,
}
