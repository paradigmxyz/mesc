use crate::{Cli, HelpArgs, MescCliError};
use clap::CommandFactory;
use toolstr::Colorize;

pub(crate) fn get_help_topics() -> Vec<(String, String)> {
    vec![("env".to_string(), "Description of environmental variables".to_string())]
}

pub(crate) fn help_command(args: HelpArgs) -> Result<(), MescCliError> {
    match args.topic.map(|s| s.to_lowercase()) {
        Some(topic) => match topic.as_str() {
            "env" => print_env_help(),
            _ => println!("Unknown help topic: {}", topic),
        },
        None => Cli::command().print_help().map_err(|e| {
            let message = format!("Failed to print help: {}", e);
            MescCliError::Error(message)
        })?,
    }
    Ok(())
}

fn print_env_help() {
    let messages = vec![
        (
            "MESC_MODE",
            format!(
                "{}, {}, or {}",
                "PATH".bold().green(),
                "ENV".bold().green(),
                "DISABLED".bold().green()
            ),
            "PATH",
        ),
        ("MESC_PATH", "path to MESC config file".to_string(), "/path/to/config/mesc.json"),
        ("MESC_ENV", "raw JSON MESC config data".to_string(), "{ \"endpoints\": { ... }, ... }"),
        (
            "MESC_DEFAULT_ENDPOINT",
            "url, endpoint name, or network name".to_string(),
            "local_goerli",
        ),
        (
            "MESC_NETWORK_DEFAULTS",
            format!("space-separated pairs of {}", "CHAIN_ID=ENDPOINT".bold().green()),
            "10=local_optimism 137=local_polygon",
        ),
        (
            "MESC_NETWORK_NAMES",
            format!("space-separated pairs of {}", "NETWORK_NAME=CHAIN_ID".bold().green()),
            "optimism=10 custom_fork=12345678",
        ),
        (
            "MESC_ENDPOINTS",
            format!("space-separated items of {}", "[NAME[:CHAIN_ID]=]URL".bold()),
            "local_goerli:5=localhost:8545 fork=localhost:8546",
        ),
        (
            "MESC_PROFILES",
            format!("space-separated items of {}", "PROFILE.KEY[.CHAIN_ID]=ENDPOINT".bold()),
            "tool_xyz.default_endpoint=local_goerli",
        ),
        (
            "MESC_GLOBAL_METADATA",
            "JSON-formatted global metadata".to_string(),
            "{\"tool_xyz\": { ... }}",
        ),
        (
            "MESC_ENDPOINT_METADATA",
            format!("JSON mapping of {}", "{ENDPOINT_NAME: ENDPOINT_METADATA}".bold()),
            "{\"local_goerli\": {\"api_key\": \"abc123\"}}",
        ),
    ];

    println!("MESC environment variables:");
    println!();
    for (name, value, example) in messages.iter() {
        println!("{:>22}  {}", name.bold(), value);
        println!("{:>32} {}", "example:".truecolor(100, 100, 100), example.green().bold());
        println!();
    }
}
