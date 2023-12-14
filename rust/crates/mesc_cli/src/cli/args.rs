use super::subcommands::*;
use crate::MescCliError;
use clap::{Parser, Subcommand};

pub(crate) async fn run_cli() -> Result<(), MescCliError> {
    match Cli::parse().command {
        Commands::Setup(args) => setup_command(args),
        Commands::Status(args) => status_command(args),
        Commands::Ls(args) => ls_command(args),
        Commands::Defaults(args) => defaults_command(args),
        Commands::Ping(args) => ping_command(args).await,
        Commands::Endpoint(args) => endpoint_command(args),
        Commands::Url(args) => url_command(args),
    }
}

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
    /// Create or modify configuration
    Setup(SetupArgs),
    /// Print status of configuration
    Status(StatusArgs),
    /// Print list of endpoints
    Ls(LsArgs),
    /// Print list of defaults
    Defaults(DefaultsArgs),
    /// Ping endpoints and fetch various metadata
    Ping(PingArgs),
    /// Print endpoint
    Endpoint(EndpointArgs),
    /// Print endpoint URL
    Url(UrlArgs),
}

/// Arguments for the `setup` subcommand
#[derive(Parser)]
pub struct SetupArgs {
    /// path to use
    #[clap(short, long)]
    pub path: Option<String>,

    /// edit data in editor
    #[clap(short, long)]
    pub editor: bool,
}

/// Arguments for the `status` subcommand
#[derive(Parser)]
pub struct StatusArgs {
    /// reveal all endpoint url's in output
    #[clap(short, long)]
    pub reveal: bool,

    /// verbose, show all endpoints and defaults
    #[clap(short, long)]
    pub verbose: bool,
}

/// Arguments for the `ls` subcommand
#[derive(Parser)]
pub struct LsArgs {
    /// reveal all endpoint url's in output
    #[clap(short, long)]
    pub reveal: bool,

    // /// verbose, show all endpoints and defaults
    // #[clap(short, long)]
    // pub verbose: bool,
    /// filter by name (fuzzy match)
    #[clap(long)]
    pub name: Option<String>,

    /// filter by chain id
    #[clap(short, long)]
    pub network: Option<String>,

    /// filter by url (fuzzy match)
    #[clap(short, long)]
    pub url: Option<String>,

    /// metadata, space-separated key=value pairs
    #[clap(short, long)]
    pub metadata: Vec<String>,

    /// output as json
    #[clap(long)]
    pub json: bool,
}

/// Arguments for the `ls` subcommand
#[derive(Parser)]
pub struct DefaultsArgs {
    // /// verbose, show all endpoints and defaults
    // #[clap(short, long)]
    // pub verbose: bool,
    /// output as json
    #[clap(long)]
    pub json: bool,
}

/// Arguments for the `ping` subcommand
#[derive(Parser)]
pub struct PingArgs {
    /// data fields to gather
    /// one or more of: {ip, location, latency, client, namespaces, all}
    #[clap(num_args=0.., verbatim_doc_comment)]
    pub fields: Vec<String>,

    /// filter endpoints by endpoint name (fuzzy match)
    #[clap(long)]
    pub name: Option<String>,

    /// filter endpoints by url (fuzzy match)
    #[clap(long)]
    pub url: Option<String>,

    /// filter endpoints by network
    #[clap(long)]
    pub network: Option<String>,

    /// timeout, in seconds
    #[clap(long, default_value_t = 1)]
    pub timeout: u64,
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

/// Arguments for the `url` subcommand
#[derive(Parser)]
pub struct UrlArgs {
    /// query
    #[clap()]
    pub query: Option<String>,

    /// profile
    #[clap(short, long)]
    pub profile: Option<String>,
}
