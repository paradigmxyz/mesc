use crate::{MescCliError, StatusArgs};
use mesc::MescError;

pub(crate) fn print_status(args: StatusArgs) -> Result<(), MescCliError> {
    let theme = Some(toolstr::Theme::default());

    toolstr::print_text_box("MESC Status", &theme);
    println!();
    let mut keys = Vec::new();
    let mut values = Vec::new();

    keys.push("enabled?");
    if mesc::is_mesc_enabled() {
        values.push("true".to_string());
    } else {
        values.push("false".to_string());
    }

    // print configuration mode
    match mesc::load::get_config_mode() {
        Ok(mode) => {
            keys.push("config mode");
            values.push(format!("{:?}", mode));
            // if in path mode, print path
            if let mesc::ConfigMode::Path = mode {
                match std::env::var("MESC_CONFIG_PATH") {
                    Ok(path) => {
                        keys.push("path");
                        values.push(path);
                    }
                    _ => {
                        keys.push("path");
                        values.push("[could not get path]".to_string());
                    }
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }

    // load config data
    let config = mesc::load::load_config_data();
    let config = match config {
        Err(e) => {
            keys.push("config found");
            values.push("false".to_string());
            if let MescError::IOError(ref e) = e {
                if let std::io::ErrorKind::NotFound = e.kind() {
                    println!("config file not found");
                } else {
                    println!("could not load config: {:?}", e);
                }
            } else {
                println!("could not load config: {:?}", e);
            };
            return Err(e.into());
        }
        Ok(config) => {
            keys.push("config found");
            values.push("true".to_string());
            config
        }
    };

    // validate config
    keys.push("config vaild");
    match config.validate() {
        Ok(()) => {
            values.push("true".to_string());
        }
        Err(e) => {
            values.push("false".to_string());
            println!("{:?}", e);
        }
    };

    let format = toolstr::TableFormat::default();
    // let column_formats = vec![
    //     toolstr::ColumnFormat.name("key"),
    //     toolstr::ColumnFormat.name("value"),
    // ];
    let format = toolstr::TableFormat {
        include_header_row: false,
        indent: 4,
        // column_formats,
        ..format
    };
    let mut table = toolstr::Table::default();
    table.add_column("key", keys)?;
    table.add_column("value", values)?;
    format.print(table)?;

    // print endpoint info
    println!();
    println!();
    toolstr::print_header("Configured Endpoints", &theme);
    println!();
    let reveal = if args.reveal {
        true
    } else {
        config.global_metadata.get("reveal") == Some(&serde_json::Value::Bool(true))
    };
    if config.endpoints.is_empty() {
        println!("[none]")
    } else {
        let mut endpoints = Vec::new();
        let mut networks = Vec::new();
        let mut urls = Vec::new();
        let mut sorted_endpoints: Vec<_> = config.endpoints.values().collect();
        sorted_endpoints.sort_by(|e1, e2| {
            e1.chain_id.clone()
                .unwrap_or(mesc::ChainId::null_chain_id())
                .cmp(&e2.chain_id.clone().unwrap_or(mesc::ChainId::null_chain_id()))
        });
        for endpoint in sorted_endpoints.into_iter() {
            endpoints.push(endpoint.name.clone());
            networks.push(endpoint.chain_id_string());
            if reveal {
                urls.push(endpoint.url.clone());
            } else {
                urls.push("*".repeat(8));
            }
        }
        let format = toolstr::TableFormat::default();
        let format = toolstr::TableFormat {
            // indent: 4,
            // column_delimiter: " . ".to_string(),
            // header_separator_delimiter: " . ".to_string(),
            ..format
        };
        let mut table = toolstr::Table::default();
        table.add_column("endpoint", endpoints)?;
        table.add_column("network", networks)?;
        table.add_column("url", urls)?;
        format.print(table)?;
    };

    // print defaults
    println!();
    println!();
    toolstr::print_header("Default Endpoints", &theme);
    println!();
    let mut classes = Vec::new();
    let mut networks = Vec::new();
    let mut names = Vec::new();
    classes.push("global default");
    if let Some(default_endpoint) = mesc::get_default_endpoint(None)? {
        names.push(default_endpoint.name.clone());
        networks.push(default_endpoint.chain_id_string());
    }
    for (chain_id, name) in config.network_defaults.iter() {
        classes.push("network default");
        networks.push(chain_id.to_string());
        names.push(name.clone());
    }
    let format = toolstr::TableFormat::default();
    let format = toolstr::TableFormat {
        // indent: 4,
        ..format
    };
    let mut table = toolstr::Table::default();
    table.add_column("", classes)?;
    table.add_column("network", networks)?;
    table.add_column("endpoint", names)?;
    format.print(table)?;

    if config.profiles.is_empty() {
        // println!();
        // println!();
        // println!("[none]");
    } else {
        println!();
        println!();
        toolstr::print_header("Additional Profiles", &theme);
        for (name, _profile) in config.profiles.iter() {
            println!("- {}", name);
        }
    };

    Ok(())
}
