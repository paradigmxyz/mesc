use crate::MescCliError;
use inquire::InquireError;
use mesc::RpcConfig;

use super::{defaults::*, endpoints::*, metadata::*, writing::*};
use toolstr::Colorize;
use super::network_names::*;

pub(crate) async fn modify_existing_config(
    config: RpcConfig,
    config_write_mode: Option<ConfigWriteMode>,
) -> Result<(), MescCliError> {
    let options = [
        "Add new endpoint",
        "Modify endpoint",
        "Modify defaults",
        "Modify metadata",
        "Modify network names",
        "Print config as JSON",
        "Exit and save changes",
        "Exit without saving",
    ]
    .to_vec();
    let original_config = config.clone();
    let mut valid_config = config.clone();
    let mut config = config.clone();
    loop {
        // modify config
        match inquire::Select::new("What do you want to do?", options.clone())
            .with_page_size(10)
            .prompt()
        {
            Ok("Add new endpoint") => add_endpoint(&mut config).await?,
            Ok("Modify endpoint") => modify_endpoint(&mut config).await?,
            Ok("Modify defaults") => modify_defaults(&mut config)?,
            Ok("Modify metadata") => modify_global_metadata(&mut config).await?,
            Ok("Modify network names") => modify_custom_network_names(&mut config).await?,
            Ok("Print config as JSON") => {
                println!();
                println!("{}", colored_json::to_colored_json_auto(&config)?);
                println!();
            }
            Ok("Exit and save changes") => break,
            Ok("Exit without saving") => return Ok(()),
            Err(InquireError::OperationCanceled) => {
                if config != original_config {
                    println!(" Exiting without saving");
                } else {
                    println!(" Exiting");
                }
                std::process::exit(0)
            }
            Ok(_) | Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        };

        // validation
        match config.validate() {
            Ok(()) => {}
            Err(e) => {
                println!("Invalid data: {:?}", e);
                println!("Reverting to previous config state");
                config = valid_config.clone();
                continue;
            }
        };

        valid_config = config.clone();
    }

    // write file
    if config != original_config {
        write_config(config, config_write_mode)?;
    } else {
        println!(" {}", "No updates to save".bold())
    }
    Ok(())
}
