use super::subcommands::*;
use crate::MescCliError;
use clap::{Parser, Subcommand};

pub(crate) async fn run_cli() -> Result<(), MescCliError> {
    match Cli::parse().command {
        Commands::Setup(args) => setup_command(args),
        Commands::Status(args) => status_command(args),
        Commands::Ls(args) => ls_command(args),
        Commands::Defaults(args) => defaults_command(args),
        Commands::Find(args) => find_command(args),
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
    /// Create new configuration
    Setup(SetupArgs),
    /// Print status of configuration
    Status(StatusArgs),
    /// Print list of endpoints
    Ls(LsArgs),
    /// Print list of defaults
    Defaults(DefaultsArgs),
    /// Search through list of configured endpoints
    Find(FindArgs),
    /// Ping endpoints and get their client versions
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
    #[clap(long)]
    pub reveal: bool,

    /// verbose, show all endpoints and defaults
    #[clap(short, long)]
    pub verbose: bool,
}

/// Arguments for the `ls` subcommand
#[derive(Parser)]
pub struct LsArgs {
    /// reveal all endpoint url's in output
    #[clap(long)]
    pub reveal: bool,

    // /// verbose, show all endpoints and defaults
    // #[clap(short, long)]
    // pub verbose: bool,

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
    #[clap(short, long)]
    pub query: Option<String>,

    /// profile
    #[clap(short, long)]
    pub profile: Option<String>,
}

