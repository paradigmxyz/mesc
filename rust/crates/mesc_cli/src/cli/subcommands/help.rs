use crate::{Cli, HelpArgs, MescCliError};
use clap::CommandFactory;
use toolstr::Colorize;

pub(crate) fn help_command(args: HelpArgs) -> Result<(), MescCliError> {
    match args.topic.map(|s| s.to_lowercase()) {
        Some(topic) => match topic.as_str() {
            "env" => print_env_help(),
            "python" => print_python_interface(),
            "rust" => print_rust_interface(),
            "schema" => print_schema()?,
            "setup" => print_setup_help(),
            _ => println!("Unknown help topic: {}", topic),
        },
        None => Cli::command().print_help().map_err(|e| {
            let message = format!("Failed to print help: {}", e);
            MescCliError::Error(message)
        })?,
    }
    Ok(())
}

fn print_help_topic(topic: &str) -> Result<(), MescCliError> {
    match topic {
        "env" => print_env_help(),
        "python" => print_python_interface(),
        "rust" => print_rust_interface(),
        "schema" => print_schema()?,
        "setup" => print_setup_help(),
        _ => println!("Unknown help topic: {}", topic),
    }
    Ok(())
}

pub(crate) fn get_help_topics() -> Vec<(String, String)> {
    vec![
        ("env".to_string(), "Environmental variables".to_string()),
        ("python".to_string(), "Python interface".to_string()),
        ("rust".to_string(), "Rust interface".to_string()),
        ("schema".to_string(), "Schemas of configs, endpoints, and profiles".to_string()),
        ("setup".to_string(), "How to set up MESC".to_string()),
    ]
}

pub(crate) fn print_interactive_help() -> Result<(), MescCliError> {
    println!();
    print_setup_help();
    println!();
    let topics = get_help_topics();
    let mut options: Vec<_> = topics.iter().map(|(t, _)| t.as_str()).collect();
    options.push("Return to main menu");
    loop {
        let prompt = "Print help about other MESC topics?";
        match inquire::Select::new(prompt, options.clone()).prompt() {
            Ok("Return to main menu") => return Ok(()),
            Ok(topic) => {
                println!();
                print_help_topic(topic)?;
                println!()
            }
            Err(inquire::InquireError::OperationCanceled) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
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
    for (name, value, example) in messages.iter() {
        println!();
        println!("{:>22}  {}", name.bold(), value);
        println!("{:>32} {}", "example:".truecolor(100, 100, 100), example.green().bold());
    }
}

fn print_code(content: String, filetype: &str) {
    use bat::PrettyPrinter;

    PrettyPrinter::new()
        .input_from_bytes(content.as_bytes())
        .language(filetype)
        // .theme("Visual Studio Dark+")
        // .theme("Monokai Extended Bright")
        .theme("Dracula")
        // .theme("DarkNeon")
        // .theme("Coldark-Dark")
        // .theme("base16-256")
        .print()
        .unwrap();
    println!();
}

fn print_python_interface() {
    let interface = r#"from typing import Any, Mapping, Sequence
import mesc

# check whether mesc is enabled
enabled: bool = mesc.is_mesc_enabled()

# get the default endpoint
endpoint: Endpoint | None = mesc.get_default_endpoint()

# get the default endpoint of a network
endpoint: Endpoint | None = mesc.get_endpoint_by_network(5)

# get the default endpoint for a particular tool
endpoint: Endpoint | None = mesc.get_default_endpoint(profile='xyz_tool')

# get the default endpoint of a network for a particular tool
endpoint: Endpoint | None = mesc.get_endpoint_by_network(5, profile='xyz_tool')

# get an endpoint by name
endpoint: Endpoint | None = mesc.get_endpoint_by_name('local_goerli')

# parse a user-provided string into a matching endpoint
# (try 1. endpoint name, then 2. chain id, then 3. network name)
endpoint: Endpoint | None = mesc.get_endpoint_by_query(user_str, profile='xyz_tool')

# find all endpoints matching given criteria
endpoints: Sequence[Endpoint] = mesc.find_endpoints(chain_id=5)

# get non-endpoint metadata
metadata: Mapping[str, Any] = mesc.get_global_metadata(profile='xyz_tool')"#;

    print_code(interface.to_string(), "py")
}

fn print_rust_interface() {
    let interface = r#"use mesc;
use mesc::MescError;

type OptionalResult = Result<Option<Endpoint>, MescError>;
type MultiResult = Result<Vec<Endpoint>, MescError>;
type MetadataResult = Result<HashMap<String, serde_json::Value>, MescError>

// get the default endpoint
let endpoint: OptionalResult = mesc::get_default_endpoint(None);

// get the default endpoint of a network
let endpoint: OptionalResult = mesc::get_endpoint_by_network(5, None);

// get the default network for a particular tool
let chain_id: OptionalResult = mesc::get_default_endpoint(Some("xyz_tool"));

// get the default endpoint of a network for a particular tool
let endpoint: OptionalResult = mesc::get_endpoint_by_network(5, Some("xyz_tool"));

// get an endpoint by name
let endpoint: OptionalResult = mesc::get_endpoint_by_name("local_goerli");

// parse a user-provided string into a matching endpoint
// (first try 1. endpoint name, then 2. chain id, then 3. network name)
let endpoint: OptionalResult = mesc::get_endpoint_by_query(user_str, Some("xyz_tool"));

// find all endpoints matching given criteria
let query = mesc::MultiEndpointQuery::new().chain_id(5);
let endpoints: MultiResult = mesc::find_endpoints(query);

// get non-endpoint metadata
let metadata: MetadataResult  = mesc::get_global_metadata(Some("xyz_tool"));"#;

    print_code(interface.to_string(), "rs")
}

fn print_schema() -> Result<(), MescCliError> {
    let mut title_style = crate::metadata::get_theme_font_style("title")?;
    title_style.bold();
    let metavar_style = crate::metadata::get_theme_font_style("metavar")?;
    let mut description_style = crate::metadata::get_theme_font_style("description")?;
    description_style.bold();
    let option_style = crate::metadata::get_theme_font_style("option")?;
    let _content_style = crate::metadata::get_theme_font_style("content")?;
    let comment_style = crate::metadata::get_theme_font_style("comment")?;

    let keys = vec![
        "mesc_version",
        "default_endpoint",
        "network_defaults",
        "network_names",
        "endpoints",
        "profiles",
        "global_metadata",
    ];
    let types = vec![
        "str",
        "str | None",
        "Mapping[ChainId, str",
        "Mapping[str, ChainId]",
        "Mapping[str, ChainId]",
        "Mapping[str, ChainId]",
        "Mapping[str, ChainId]",
    ];
    let descriptions = vec![
        "must equal the value \"MESC 1.0\"",
        "name of default endpoint",
        "map of chain_id's to endpoint names",
        "map of network names to chain_id's",
        "map of endpoint names to endpoints",
        "map of profile names to profiles",
        "global metadata entries",
    ];
    let mut table = toolstr::Table::default();
    table.add_column("key", keys)?;
    table.add_column("type", types)?;
    table.add_column("description", descriptions)?;
    let mut format = toolstr::TableFormat::default()
        .border_font_style(comment_style.clone())
        .label_font_style(title_style.clone());
    format.add_column(
        toolstr::ColumnFormatShorthand::new().name("key").font_style(metavar_style.clone()),
    );
    format.add_column(
        toolstr::ColumnFormatShorthand::new().name("type").font_style(option_style.clone()),
    );
    format.add_column(
        toolstr::ColumnFormatShorthand::new()
            .name("description")
            .font_style(description_style.clone()),
    );
    let theme = toolstr::Theme::default()
        .with_border_style(comment_style.clone())
        .with_title_style(title_style.clone());
    toolstr::print_text_box("RPC Config", theme);
    format.print(table)?;

    let keys = vec!["name", "url", "chain_id", "endpoint_metadata"];
    let types = vec!["str", "str", "ChainId | None", "Mapping[str, Any]"];
    let descriptions = vec![
        "name of endpoint",
        "url of endpoint, including transport",
        "chain id of network",
        "endpoint metadata entries",
    ];
    let mut table = toolstr::Table::default();
    table.add_column("key", keys)?;
    table.add_column("type", types)?;
    table.add_column("description", descriptions)?;
    let mut format = toolstr::TableFormat::default()
        .border_font_style(comment_style.clone())
        .label_font_style(title_style.clone());
    format.add_column(
        toolstr::ColumnFormatShorthand::new().name("key").font_style(metavar_style.clone()),
    );
    format.add_column(
        toolstr::ColumnFormatShorthand::new().name("type").font_style(option_style.clone()),
    );
    format.add_column(
        toolstr::ColumnFormatShorthand::new()
            .name("description")
            .font_style(description_style.clone()),
    );
    let theme = toolstr::Theme::default()
        .with_border_style(comment_style.clone())
        .with_title_style(title_style.clone());
    println!();
    println!();
    toolstr::print_text_box("Endpoint", theme);
    format.print(table)?;

    let keys = vec!["name", "default_endpoint", "network_defaults"];
    let types = vec!["str", "str | None", "Mapping[ChainId, str]"];
    let descriptions = vec![
        "name of profile",
        "chain_id of default network",
        "map of chain_id's to endpoint names",
    ];
    let mut table = toolstr::Table::default();
    table.add_column("key", keys)?;
    table.add_column("type", types)?;
    table.add_column("description", descriptions)?;
    let mut format = toolstr::TableFormat::default()
        .border_font_style(comment_style.clone())
        .label_font_style(title_style.clone());
    format.add_column(
        toolstr::ColumnFormatShorthand::new().name("key").font_style(metavar_style.clone()),
    );
    format.add_column(
        toolstr::ColumnFormatShorthand::new().name("type").font_style(option_style.clone()),
    );
    format.add_column(
        toolstr::ColumnFormatShorthand::new()
            .name("description")
            .font_style(description_style.clone()),
    );
    let theme = toolstr::Theme::default()
        .with_border_style(comment_style.clone())
        .with_title_style(title_style.clone());
    println!();
    println!();
    toolstr::print_text_box("RPC Config", theme);
    format.print(table)?;

    Ok(())
}

fn print_setup_help() {
    println!(
        r#"A basic MESC setup requires two steps:
1. create a {} configuration file
2. set the {} environment variable to the path of this file

The {} command can interactively perform both steps

{} can also be used to create and modify configuration data"#,
        "mesc.json".white().bold(),
        "MESC_PATH".white().bold(),
        "mesc setup".white().bold(),
        "mesc setup".white().bold()
    )
}
