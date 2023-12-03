use mesc::MescError;

pub(crate) fn print_status() {
    if mesc::is_mesc_enabled() {
        println!("MESC is enabled");
    } else {
        println!("MESC not enabled");
    }

    // print configuration mode
    match mesc::load::get_config_mode() {
        Ok(mode) => {
            println!("- config mode: {:?}", mode);
            // if in path mode, print path
            if let mesc::ConfigMode::Path = mode {
                match std::env::var("MESC_CONFIG_PATH") {
                    Ok(path) => println!("- path: {}", path),
                    _ => println!("- path: [could not get path]"),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }

    // load config data
    let config = mesc::load::load_config_data();
    let config = match config {
        Err(e) => {
            println!("- config found: false");
            println!();
            if let MescError::FileReadError(e) = e {
                if let std::io::ErrorKind::NotFound = e.kind() {
                    println!("config file not found");
                } else {
                    println!("could not load config: {:?}", e);
                }
            } else {
                println!("could not load config: {:?}", e);
            };
            return;
        }
        Ok(config) => {
            println!("- config found: true");
            config
        },
    };

    // validate config
    match config.validate() {
        Ok(()) => println!("- config valid: true"),
        Err(e) => {
            println!("- config valid: false");
            println!();
            println!("{:?}", e);
        },
    };

    // print endpoint info
    println!("Endpoints");
    if config.endpoints.is_empty() {
        println!("[none]")
    } else {
        println!()
    };

    // print defaults
    println!();
    println!("Default endpoints");
    println!(
        "- default endpoint: {}",
        config.default_endpoint.unwrap_or("[none]".into())
    );
    println!();
    if config.network_defaults.is_empty() {
        println!("- network_defaults: [none]")
    } else {
        println!("- network defaults:");
        for (name, chain_id) in config.network_defaults.iter() {
            println!("    - {}: {}", name, chain_id);
        }
    };
    println!("- additional profiles: {}", config.profiles.len());
}

