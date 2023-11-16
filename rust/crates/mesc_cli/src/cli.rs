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
    /// Print endpoint as JSON
    Json(JsonArgs),
    /// Print endpoint as human readble
    Pretty(PrettyArgs),
    /// Print entire configuration
    Find(FindArgs),
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
pub struct JsonArgs {
    /// query
    #[clap(short, long)]
    pub query: Option<String>,

    /// profile
    #[clap(short, long)]
    pub profile: Option<String>,
}

/// Arguments for the `pretty` subcommand
#[derive(Parser)]
pub struct PrettyArgs {
    /// query
    #[clap(short, long)]
    pub query: Option<String>,

    /// profile
    #[clap(short, long)]
    pub profile: Option<String>,
}

/// Arguments for the `all` subcommand
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

