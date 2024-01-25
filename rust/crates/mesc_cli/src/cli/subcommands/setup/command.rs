use crate::{MescCliError, SetupArgs};
use mesc::{MescError, RpcConfig};
use toolstr::Colorize;

use super::{config_modification::*, inquire_utils::*, selectors::*, shell_config::*, writing::*};

pub(crate) async fn setup_command(args: SetupArgs) -> Result<(), MescCliError> {
    inquire::set_global_render_config(get_render_config());
    if args.editor {
        edit_config_in_editor(args)
    } else {
        let mode = get_write_mode()?;
        let config = load_config_data(&mode)?;
        let endpoint_word = if config.endpoints.len() == 1 { "endpoint" } else { "endpoints" };
        let profile_word = if config.profiles.len() == 1 { "profile" } else { "profiles" };
        println!(
            " Current config has {} {} and {} {}",
            config.endpoints.len().to_string().green(),
            endpoint_word,
            config.profiles.len().to_string().green(),
            profile_word,
        );
        println!();
        modify_existing_config(config, Some(mode)).await
    }
}

/// edit MESC config in editor instead of interactive menus
fn edit_config_in_editor(args: SetupArgs) -> Result<(), MescCliError> {
    if let Some(path) = args.path {
        edit::edit_file(path)?
    } else if let Ok(mesc::ConfigMode::Path) = mesc::load::get_config_mode() {
        let path = mesc::load::get_config_path()?;
        edit::edit_file(path)?
    } else {
        return Err(MescCliError::Error("no file to edit".to_string()));
    };
    Ok(())
}

/// get write mode for config
fn get_write_mode() -> Result<ConfigWriteMode, MescCliError> {
    match (
        std::env::var("MESC_MODE").as_deref(),
        std::env::var("MESC_PATH").as_deref(),
        std::env::var("MESC_ENV").as_deref(),
        !mesc::overrides::get_active_overrides().is_empty(),
    ) {
        (Ok("DISABLED"), _, _, _) => {
            println!(" MESC is disabled because {}", "MESC_MODE=DISABLED".green().bold());
            println!(
                " To enable MESC, set {} to {} or {} or unset it",
                "MESC_MODE".green().bold(),
                "PATH".green().bold(),
                "ENV".green().bold()
            );
            let config_mode = select_config_mode()?;
            setup_mesc_mode_env_var(&config_mode)?;
            setup_mesc_env_vars(&config_mode)?;
            Ok(config_mode)
        }
        (Ok("PATH"), Ok(path), _, _) | (Err(_), Ok(path), _, _) => {
            let path = mesc::load::expand_path(path)?;
            println!(" MESC is {}", "enabled".green().bold());
            let print_path =
                if let Ok(raw_path) = std::env::var("MESC_PATH") { raw_path } else { path.clone() };
            println!(" Using {}{}", "MESC_PATH=".green().bold(), print_path.green().bold());

            Ok(ConfigWriteMode::Path(path.into()))
        }
        (Ok("PATH"), Err(_), _, _) => {
            println!(" MESC is enabled");
            println!(
                " Using {}, but {} is not set",
                "MESC_MODE=PATH".green().bold(),
                "MESC_PATH".green().bold()
            );
            let path = select_config_path()?;
            println!(" Using config path: {}", path.to_string_lossy().green().bold());
            setup_mesc_path_env_var(&path)?;
            Ok(ConfigWriteMode::Path(path))
        }
        (Ok("ENV"), _, Ok(_env), _) | (Err(_), Err(_), Ok(_env), _) => {
            println!(" MESC is enabled");
            println!(" Using {}", "MESC_MODE=ENV".green().bold());
            Ok(ConfigWriteMode::Env(vec![]))
        }
        (Ok("ENV"), _, Err(_), _) => {
            println!(" MESC is enabled");
            println!(
                " Using {}, but {} is not set",
                "MESC_MODE=ENV".green().bold(),
                "MESC_ENV".green().bold()
            );
            Ok(ConfigWriteMode::Env(vec![]))
        }
        (Ok(other), _, _, _) => {
            eprintln!("Invaild value for {}: {}", "MESC_MODE".green().bold(), other.green().bold());
            eprintln!(
                "Either 1) unset this variable, or 2) set it to one of {{PATH, ENV, DISABLED}}"
            );
            Err(MescCliError::Error("invalid MESC_MODE".to_string()))
        }
        (Err(_), Err(_), Err(_), true) => {
            println!(" MESC is only enabled because overrides are set:");
            for var_name in mesc::overrides::get_active_overrides().iter() {
                println!("- {}", var_name.bold().green())
            }
            println!(
                " To enable MESC more permanently, set either {} or {}",
                "MESC_PATH".green().bold(),
                "MESC_ENV".green().bold()
            );
            let mode = select_config_mode()?;
            setup_mesc_env_vars(&mode)?;
            Ok(mode)
        }
        (Err(_), Err(_), Err(_), false) => {
            println!(" MESC is disabled because no MESC env vars are set");
            println!(" To enabled MESC, set one of the MESC env vars");
            let mode = select_config_mode()?;
            setup_mesc_env_vars(&mode)?;
            Ok(mode)
        }
    }
}

/// load config data based on write mode
fn load_config_data(mode: &ConfigWriteMode) -> Result<RpcConfig, MescCliError> {
    // attempt to load config data
    let config = match mode {
        ConfigWriteMode::Path(path) => {
            // if path DNE that is ok, just start from scratch
            // if invalid json, or other io error, ask to start from scratch break
            let option_path = Some(path.to_string_lossy().into());
            match mesc::load::load_file_config(option_path) {
                Ok(config) => Ok(config),
                Err(MescError::MissingConfigFile(_)) => {
                    let config = RpcConfig::default();
                    let prompt = "Config file does not exist, do you want to create one?";
                    match inquire::Confirm::new(prompt).with_default(true).prompt() {
                        Ok(true) => {
                            mesc::write::write_config(config.clone(), path)?;
                            println!(
                                " Created blank config at {}",
                                path.to_string_lossy().bold().green()
                            );
                        }
                        Ok(false) => {
                            println!(" Starting from blank config. Use \"Exit and save changes\" below to save");
                        }
                        Err(_) => std::process::exit(1),
                    }
                    Ok(config)
                }
                Err(e) => Err(e),
            }
        }
        ConfigWriteMode::Env(_) => {
            // if env unset that is ok, just start from scratch
            // if invalid json, ask to start from scratch, or break
            match std::env::var("MESC_ENV") {
                Ok(val) if !val.is_empty() => mesc::load::load_env_config(),
                _ => {
                    println!(" {} var is empty, using blank config", "MESC_ENV".green().bold());
                    Ok(RpcConfig::default())
                }
            }
        }
    };

    // create new config upon loading error
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Could not load config data: {}", format!("{}", e).red());
            let prompt = "How do you want to procede?";
            let options = vec!["Start a new config from scratch", "Exit setup"];
            match inquire::Select::new(prompt, options).prompt() {
                Ok("Start a new config from scratch") => RpcConfig::default(),
                _ => std::process::exit(1),
            }
        }
    };

    // ensure that config data is valid
    let mut config = config.clone();
    loop {
        match config.validate() {
            Ok(()) => break,
            Err(e) => {
                eprintln!("Config data is not valid: {}", format!("{}", e).red());
                let prompt = "What do you want to do?";
                let options =
                    vec!["Create new config from scratch", "Edit the data manually", "Exit setup"];
                match inquire::Select::new(prompt, options).prompt() {
                    Ok("Create new config from scratch") => config = RpcConfig::default(),
                    Ok("Edit the data manually") => {
                        let new_config = edit::edit(serde_json::to_string(&config)?)?;
                        if let Ok(new_config) = serde_json::from_str(new_config.as_str()) {
                            config = new_config
                        }
                    }
                    Ok("Exit setup") | Ok(_) | Err(_) => std::process::exit(1),
                }
            }
        };
    }

    Ok(config)
}

fn setup_mesc_mode_env_var(mode: &ConfigWriteMode) -> Result<(), MescCliError> {
    let value = match mode {
        ConfigWriteMode::Path(_) => "PATH",
        ConfigWriteMode::Env(_) => "ENV",
    };
    modify_shell_config_var("MESC_MODE", value.to_string(), None)?;
    Ok(())
}

fn setup_mesc_env_vars(mode: &ConfigWriteMode) -> Result<(), MescCliError> {
    match mode {
        ConfigWriteMode::Path(path) => setup_mesc_path_env_var(path),
        ConfigWriteMode::Env(_) => Ok(()),
    }
}

fn setup_mesc_path_env_var(path: &std::path::Path) -> Result<(), MescCliError> {
    let value = path.to_string_lossy().to_string();
    modify_shell_config_var("MESC_PATH", value, None)?;
    Ok(())
}
