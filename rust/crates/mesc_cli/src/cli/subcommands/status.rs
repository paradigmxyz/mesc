use crate::{print_defaults, print_endpoints, MescCliError, StatusArgs};
use mesc::MescError;
use toolstr::Colorize;

pub(crate) fn status_command(args: StatusArgs) -> Result<(), MescCliError> {
    let theme = Some(toolstr::Theme::default());

    toolstr::print_text_box("MESC Status", &theme);
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
                match std::env::var("MESC_PATH") {
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
                    println!("{}", "config file not found".red());
                    println!();
                } else {
                    println!("{}", format!("could not load config: {:?}", e).red());
                    println!();
                    keys.push("config found");
                    values.push(format!("{}", "false".to_string().red()));
                }
            } else {
                println!("{}", format!("could not load config: {:?}", e).red());
                println!();
            };
            None
        }
        Ok(config) => {
            keys.push("config found");
            values.push(format!("{}", "true".to_string().green()));
            Some(config)
        }
    };

    // validate config
    keys.push("config valid");
    if let Some(config) = config.clone() {
        match config.validate() {
            Ok(()) => {
                values.push(format!("{}", "true".to_string().green()));
            }
            Err(e) => {
                values.push(format!("{}", "false".to_string().red()));
                println!("{:?}", e);
            }
        };
    } else {
        values.push("[no config]".to_string());
    }

    let format = toolstr::TableFormat::default();
    let column_formats = vec![
        toolstr::ColumnFormatShorthand::new().name("key"),
        toolstr::ColumnFormatShorthand::new()
            .name("v")
            .left_justify()
            .font_style("".bold().green()),
    ];
    let format = toolstr::TableFormat {
        include_header_row: false,
        indent: 4,
        column_formats: Some(column_formats),
        column_delimiter: "  ".to_string(),
        ..format
    };
    let mut table = toolstr::Table::default();
    table.add_column("key", keys)?;
    table.add_column("v", values)?;
    format.print(table)?;

    println!();
    crate::printing::print_environment_variables(0);

    // print endpoint info
    if args.verbose {
        println!();
        println!();
        toolstr::print_header("Configured Endpoints", &theme);
        println!();
        if let Some(ref config) = config.clone() {
            let endpoints: Vec<_> = config.endpoints.clone().into_values().collect();
            print_endpoints(&endpoints, args.reveal)?;
        } else {
            println!("[no config]");
        }
    };

    // print defaults
    if args.verbose {
        println!();
        println!();
        toolstr::print_header("Default Endpoints", &theme);
        println!();
        if let Some(config) = config.clone() {
            print_defaults(&config)?;
        } else {
            println!("[no config]");
        }
    };

    if !args.verbose {
        println!();
        println!(
            "use {} or {} to print additional information",
            "-v".bold().white(),
            "--verbose".bold().white()
        )
    }

    Ok(())
}
