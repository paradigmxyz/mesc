use super::subcommands::*;
use crate::MescCliError;
use clap::{Parser, Subcommand};
use toolstr::Colorize;

pub(crate) async fn run_cli() -> Result<(), MescCliError> {
    match Cli::parse().command {
        Commands::Setup(args) => setup_command(args).await,
        Commands::Import(args) => import_command(args).await,
        Commands::Set(args) => set_command(args).await,
        Commands::Status(args) => status_command(args),
        Commands::Ls(args) => ls_command(args),
        Commands::Defaults(args) => defaults_command(args),
        Commands::Ping(args) => ping_command(args).await,
        Commands::Endpoint(args) => endpoint_command(args),
        Commands::Metadata(args) => metadata_command(args),
        Commands::Url(args) => url_command(args),
        Commands::Help(args) => help_command(args),
    }
}

fn get_after_str() -> String {
    let example = format!(
        "{} {}{}",
        "(print these with".italic().truecolor(100, 100, 100),
        "mesc help <TOPIC>".bold(),
        ")".italic().truecolor(100, 100, 100),
    );
    let mut output = format!("{} {}", "Help topics:".to_string().bold().underline(), example);
    for (topic, description) in get_help_topics().iter() {
        output = format!("{}\n  {:<9} {}", output, topic.bold(), description);
    }
    output
}

/// Utility for creating and managing MESC RPC configurations
#[derive(Parser)]
#[clap(author, version, about, long_about = None, disable_help_subcommand = true, after_help=&get_after_str())]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

/// Define your subcommands as an enum
#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Create or modify config interactively
    Setup(SetupArgs),
    /// Modify config by importing from file or other source
    Import(ImportArgs),
    /// Modify config by setting specific values
    ///
    /// This command is idempotent
    Set(SetArgs),
    /// Ping endpoints and fetch metadata
    Ping(PingArgs),
    /// Print list of defaults
    Defaults(DefaultsArgs),
    /// Print endpoint
    Endpoint(EndpointArgs),
    /// Print help
    Help(HelpArgs),
    /// Print list of endpoints
    Ls(LsArgs),
    /// Print metadata
    Metadata(MetadataArgs),
    /// Print status of configuration
    Status(StatusArgs),
    /// Print endpoint URL
    Url(UrlArgs),
}

/// Arguments for the `setup` subcommand
#[derive(Parser)]
pub(crate) struct SetupArgs {
    /// path to use
    #[clap(short, long)]
    pub(crate) path: Option<String>,

    /// edit data in editor
    #[clap(short, long)]
    pub(crate) editor: bool,
}

/// Arguments for the `import` subcommand
#[derive(Parser)]
pub(crate) struct ImportArgs {
    /// source for important
    #[clap()]
    pub(crate) source: Option<String>,

    /// interactively decide how to perform import
    #[clap(short, long)]
    pub(crate) interactive: bool,

    /// specify source name
    #[clap(long)]
    pub(crate) source_name: Option<String>,

    /// only import endpoints from this network
    #[clap(long)]
    pub(crate) network: Option<String>,

    /// only endpoint with this name
    #[clap(long)]
    pub(crate) name: Option<String>,

    /// output filepath (default = MESC_PATH)
    #[clap(short, long)]
    pub(crate) output_path: Option<String>,
}

/// Arguments for the `set` subcommand
#[derive(Parser)]
pub(crate) struct SetArgs {
    /// key to set
    #[clap()]
    pub(crate) key: Option<String>,

    /// value to set
    #[clap()]
    pub(crate) value: Option<String>,

    /// space-separated list of keys
    #[clap(long)]
    pub(crate) keys: Option<Vec<String>>,

    /// space-separated list of values (must match --keys)
    #[clap(long)]
    pub(crate) values: Option<Vec<String>>,

    /// delete specified key(s) instead of setting them
    #[clap(long)]
    pub(crate) delete: bool,

    /// config path to use [default: MESC_PATH]
    #[clap(long)]
    pub(crate) path: Option<String>,
}

/// Arguments for the `status` subcommand
#[derive(Parser)]
pub(crate) struct StatusArgs {
    /// reveal all endpoint url's in output
    #[clap(short, long)]
    pub(crate) reveal: bool,

    /// verbose, show all endpoints and defaults
    #[clap(short, long)]
    pub(crate) verbose: bool,
}

/// Arguments for the `ls` subcommand
#[derive(Parser)]
pub(crate) struct LsArgs {
    /// reveal all endpoint url's in output
    #[clap(short, long)]
    pub(crate) reveal: bool,

    /// filter by name (fuzzy match)
    #[clap(long)]
    pub(crate) name: Option<String>,

    /// filter by chain id
    #[clap(long)]
    pub(crate) network: Option<String>,

    /// filter by url (fuzzy match)
    #[clap(long)]
    pub(crate) url: Option<String>,

    /// metadata, space-separated key=value pairs
    #[clap(long)]
    pub(crate) metadata: Vec<String>,

    /// output as json
    #[clap(long)]
    pub(crate) json: bool,

    /// output url's only (space-separate)
    #[clap(long)]
    pub(crate) urls: bool,
}

/// Arguments for the `ls` subcommand
#[derive(Parser)]
pub(crate) struct DefaultsArgs {
    /// output as json
    #[clap(long)]
    pub(crate) json: bool,
}

/// Arguments for the `ping` subcommand
#[derive(Parser)]
pub(crate) struct PingArgs {
    /// data fields to gather
    /// one or more of: {ip, location, latency, client, namespaces, all}
    #[clap(num_args=0.., verbatim_doc_comment)]
    pub(crate) fields: Vec<String>,

    /// filter endpoints by endpoint name (fuzzy match)
    #[clap(long)]
    pub(crate) name: Option<String>,

    /// filter endpoints by url (fuzzy match)
    #[clap(long)]
    pub(crate) url: Option<String>,

    /// filter endpoints by network
    #[clap(long)]
    pub(crate) network: Option<String>,

    /// timeout, in seconds
    #[clap(long, default_value_t = 1)]
    pub(crate) timeout: u64,

    /// output as json
    #[clap(long)]
    pub(crate) json: bool,
}

/// Arguments for the `json` subcommand
#[derive(Parser)]
pub(crate) struct EndpointArgs {
    /// query
    #[clap()]
    pub(crate) query: Option<String>,

    /// name
    #[clap(long)]
    pub(crate) name: Option<String>,

    /// network
    #[clap(long)]
    pub(crate) network: Option<String>,

    /// profile
    #[clap(short, long)]
    pub(crate) profile: Option<String>,

    /// print as json
    #[clap(short, long)]
    pub(crate) json: bool,
}

/// Arguments for the `metadata` subcommand
#[derive(Parser)]
pub(crate) struct MetadataArgs {
    /// print metadata of endpoint
    #[clap(long)]
    pub(crate) endpoint: Option<String>,

    /// print metadata of profile merged with global metadata
    #[clap(long)]
    pub(crate) profile: Option<String>,

    /// print metadata of profile not merged with global metadata
    #[clap(long)]
    pub(crate) profile_only: Option<String>,
}

/// Arguments for the `url` subcommand
#[derive(Parser)]
pub(crate) struct UrlArgs {
    /// query
    #[clap()]
    pub(crate) query: Option<String>,

    /// name
    #[clap(long)]
    pub(crate) name: Option<String>,

    /// network
    #[clap(long)]
    pub(crate) network: Option<String>,

    /// profile
    #[clap(short, long)]
    pub(crate) profile: Option<String>,
}

/// Arguments for the `help` subcommand
#[derive(Parser)]
pub(crate) struct HelpArgs {
    /// topic
    #[clap()]
    pub(crate) topic: Option<String>,
}
