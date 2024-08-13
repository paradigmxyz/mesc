use crate::{network::is_ip, MescCliError};
use inquire::InquireError;
use mesc::{Endpoint, RpcConfig};
use std::collections::{HashMap, HashSet};
use toolstr::Colorize;

use super::{metadata::*, selectors::*};

pub(crate) async fn add_endpoint(config: &mut RpcConfig) -> Result<(), MescCliError> {
    let url = match inquire::Text::new("New endpoint URL?").prompt() {
        Ok(input) => input,
        Err(InquireError::OperationCanceled) => return Ok(()),
        Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    // add transport protocol
    let url = if !url.starts_with("http") & !url.starts_with("ws") & !url.ends_with(".ipc") {
        if url.starts_with("localhost") | is_ip(&url) {
            let url = format!("http://{}", url);
            println!(
                " Transport protocol not included. Defaulting to http: {}",
                url.green().bold()
            );
            url
        } else {
            let url = format!("https://{}", url);
            println!(
                " Transport protocol not included. Defaulting to https: {}",
                url.green().bold()
            );
            url
        }
    } else {
        url
    };

    // get chain_id
    println!(" Querying chain id...");
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(4)).build()?;
    let chain_id = crate::rpc::request_chain_id(client, url.clone()).await;
    let chain_id = match chain_id {
        Ok(chain_id) => {
            println!(" {} {}", "Using chain_id".bold(), chain_id.as_str().green());
            Some(chain_id)
        }
        _ => {
            println!(" {}", "Could not detect chain id".red());
            select_chain_id("How to proceed?".to_string()).await?
        }
    };

    // get name
    let default_name = mesc::overrides::get_default_endpoint_name(&url, chain_id.clone());
    let mut input = inquire::Text::new("New endpoint name?");
    if let Some(default_name) = default_name.as_ref() {
        input = input.with_default(default_name);
    }
    let name = match input.prompt() {
        Ok(choice) => choice,
        Err(InquireError::OperationCanceled) => return Ok(()),
        Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    // create endpoint
    let endpoint = Endpoint { url, name, chain_id, endpoint_metadata: HashMap::new() };
    config.endpoints.insert(endpoint.name.clone(), endpoint);
    println!(" {}", "New endpoint added".bold());
    Ok(())
}

pub(crate) async fn modify_endpoint(config: &mut RpcConfig) -> Result<(), MescCliError> {
    // select endpoint
    let mut options: Vec<String> = config.endpoints.clone().into_keys().collect();
    if options.is_empty() {
        println!(" No endpoints to modify");
        return Ok(());
    }
    options.sort();
    let endpoint_name = match inquire::Select::new("Which endpoint to modify?", options).prompt() {
        Ok(name) => name,
        Err(InquireError::OperationCanceled) => return Ok(()),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };
    let endpoint = match config.endpoints.get(&endpoint_name) {
        Some(endpoint) => endpoint.clone(),
        None => return Err(MescCliError::InvalidInput("endpoint does not exist".to_string())),
    };
    println!(" {}", "Current endpoint data:".bold(),);
    println!();
    let colored = colored_json::to_colored_json_auto(&serde_json::to_value(&endpoint)?)?;
    for line in colored.split('\n') {
        println!("    {}", line);
    }
    println!();

    // gather modifications
    let halt_options: HashSet<&str> = vec!["Delete endpoint", "Done"].into_iter().collect();
    let mut option = query_modify_endpoint(endpoint_name.clone(), config, true).await?;
    loop {
        if halt_options.contains(option.as_str()) {
            break;
        }
        option = query_modify_endpoint(endpoint_name.clone(), config, false).await?;
    }

    // commit modifications
    if option != "Delete endpoint" {
        let new_endpoint = match config.endpoints.get(&endpoint_name) {
            Some(endpoint) => endpoint.clone(),
            None => return Err(MescCliError::InvalidInput("endpoint does not exist".to_string())),
        };
        if new_endpoint == endpoint {
            println!(" Endpoint unmodified");
        } else {
            println!(" {}", "New endpoint data:".bold(),);
            println!();
            let colored = colored_json::to_colored_json_auto(&serde_json::to_value(endpoint)?)?;
            for line in colored.split('\n') {
                println!("    {}", line);
            }
            println!();
        }
    }

    Ok(())
}

pub(crate) async fn query_modify_endpoint(
    endpoint_name: String,
    config: &mut RpcConfig,
    first_change: bool,
) -> Result<String, MescCliError> {
    let options = [
        "Modify endpoint name",
        "Modify endpoint url",
        "Modify endpoint chain_id",
        "Modify endpoint metadata",
        "Delete endpoint",
        "Print endpoint as JSON",
        "Done",
    ]
    .to_vec();

    let message = if first_change { "How to modify endpoint?" } else { "Any other modifications?" };

    let option = match inquire::Select::new(message, options.clone()).prompt() {
        Ok(answer) => answer,
        Err(InquireError::OperationCanceled) => return Ok("Done".to_string()),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    match option {
        "Modify endpoint name" => {
            let new_name = match inquire::Text::new("New name?").prompt() {
                Ok(answer) => answer,
                Err(InquireError::OperationCanceled) => return Ok("Done".to_string()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            mesc::write::update_endpoint_name(config, endpoint_name.as_str(), new_name.as_str())?;
        }
        "Modify endpoint url" => {
            let new_url = match inquire::Text::new("New url?").prompt() {
                Ok(answer) => answer,
                Err(InquireError::OperationCanceled) => return Ok("Done".to_string()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            if let Some(endpoint) = config.endpoints.get_mut(&endpoint_name) {
                endpoint.url = new_url;
            }
        }
        "Modify endpoint chain_id" => {
            match select_chain_id("New chain_id?".to_string()).await {
                Ok(Some(chain_id)) => mesc::write::update_endpoint_chain_id(
                    config,
                    endpoint_name.as_str(),
                    chain_id.clone(),
                )?,
                Ok(None) => return Ok("Done".to_string()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
        }
        "Modify endpoint metadata" => modify_endpoint_metadata(endpoint_name.as_str(), config)?,
        "Delete endpoint" => {
            mesc::write::delete_endpoint(config, endpoint_name.as_str())?;
            println!("{} {}", "Deleted endpoint:".red(), endpoint_name.green());
        }
        "Print endpoint as JSON" => {
            let endpoint = match config.endpoints.get(&endpoint_name) {
                Some(endpoint) => endpoint.clone(),
                None => {
                    return Err(MescCliError::InvalidInput("endpoint does not exist".to_string()))
                }
            };
            println!(" {}", "Current endpoint data:".bold(),);
            println!();
            let colored = colored_json::to_colored_json_auto(&serde_json::to_value(endpoint)?)?;
            for line in colored.split('\n') {
                println!("    {}", line);
            }
            println!();
        }
        "Done" => {}
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    Ok(option.to_string())
}
