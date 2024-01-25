use crate::MescCliError;
use inquire::InquireError;
use mesc::RpcConfig;
use std::collections::HashMap;
use toolstr::Colorize;

use super::network_names::*;

pub(crate) fn modify_endpoint_metadata(
    endpoint_name: &str,
    config: &mut RpcConfig,
) -> Result<(), MescCliError> {
    let endpoint = match config.endpoints.get_mut(endpoint_name) {
        Some(endpoint) => endpoint,
        None => {
            return Err(MescCliError::InvalidInput(format!("missing endpoint: {}", endpoint_name)))
        }
    };
    let options = vec![
        "Add label",
        "Remove label",
        "Set ratelimit",
        "Set api key",
        "Edit raw JSON",
        "Done editing metadata",
    ];
    match inquire::Select::new("How to modify metadata?", options).prompt() {
        Ok("Add label") => {
            let new_label = match inquire::Text::new("What is the new label?").prompt() {
                Ok(new_label) => new_label,
                Err(InquireError::OperationCanceled) => return Ok(()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            if endpoint.endpoint_metadata.get_mut("labels").is_none() {
                endpoint
                    .endpoint_metadata
                    .insert("labels".to_string(), serde_json::json!(Vec::<String>::new()));
            };
            let labels = endpoint.endpoint_metadata.get_mut("labels").unwrap();
            if let Some(array) = labels.as_array_mut() {
                array.push(serde_json::Value::String(new_label));
                println!(" Added label")
            } else {
                println!("Incorrectly formatted data, labels is not a list");
            };
        }
        Ok("Remove label") => {
            match endpoint.endpoint_metadata.get_mut("labels") {
                Some(serde_json::Value::Array(labels)) => {
                    let label_strings: Vec<&str> =
                        labels.iter().filter_map(|val| val.as_str()).collect();
                    if label_strings.is_empty() {
                        println!("No labels for endpoint");
                        return Ok(());
                    };
                    match inquire::Select::new("Which label to remove?", label_strings.clone())
                        .prompt()
                    {
                        Ok(label) => {
                            if let Some(index) =
                                label_strings.iter().position(|test_label| test_label == &label)
                            {
                                labels.remove(index);
                            } else {
                                println!("Label not found.");
                            }
                        }
                        Err(InquireError::OperationCanceled) => return Ok(()),
                        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                    }
                }
                Some(_) => {
                    println!("Incorrectly formatted data, labels is not a list");
                }
                None => {
                    println!(" Endpoint does not have any labels")
                }
            };
        }
        Ok("Set ratelimit") => {
            let message = "What is the rate limit, in requests per second?";
            let value = match inquire::CustomType::<f64>::new(message)
                .with_formatter(&|i| format!("{:.2}", i))
                .with_error_message("Please type a valid number")
                .with_help_message(" Enter the number of requests per seconds")
                .prompt()
            {
                Ok(value) => value,
                Err(InquireError::OperationCanceled) => return Ok(()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            endpoint
                .endpoint_metadata
                .insert("rate_limit_rps".to_string(), serde_json::json!(value));
        }
        Ok("Set api key") => {
            let api_key = match inquire::Text::new("What is the api key?").prompt() {
                Ok(api_key) => api_key,
                Err(InquireError::OperationCanceled) => return Ok(()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            endpoint.endpoint_metadata.insert("api_key".to_string(), serde_json::json!(api_key));
        }
        Ok("Edit raw JSON") => {
            let edited = edit::edit(serde_json::to_string(&endpoint.endpoint_metadata)?)?;
            let value: Result<HashMap<String, serde_json::Value>, serde_json::Error> =
                serde_json::from_str(&edited);
            endpoint.endpoint_metadata = value?;
        }
        Ok("Done editing metadata") => return Ok(()),
        Err(InquireError::OperationCanceled) => return Ok(()),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
    Ok(())
}

pub(crate) async fn modify_global_metadata(config: &mut RpcConfig) -> Result<(), MescCliError> {
    let options =
        vec!["Modify raw global metadata JSON", "Edit custom network names", "Back to main menu"];
    match inquire::Select::new("What do you want to do?", options).prompt() {
        Ok("Modify raw global metadata JSON") => {
            let old_metadata = serde_json::to_string(&config.global_metadata)?;
            let new_metadata = edit::edit(&old_metadata)?;
            if old_metadata == new_metadata {
                println!(" {}", "Global metadata unchanged".bold());
            } else {
                let value: Result<HashMap<String, serde_json::Value>, serde_json::Error> =
                    serde_json::from_str(&new_metadata);
                config.global_metadata = value?;
                println!(" {}", "Global metadata updated".bold());
            }
            Ok(())
        }
        Ok("Edit custom network names") => modify_custom_network_names(config).await,
        Ok("Back to main menu") => Ok(()),
        Err(InquireError::OperationCanceled) => Ok(()),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}
