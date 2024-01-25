use crate::MescCliError;
use inquire::InquireError;
use mesc::RpcConfig;
use toolstr::Colorize;

#[derive(Clone)]
pub(crate) enum ConfigWriteMode {
    /// file path of JSON config file
    Path(std::path::PathBuf),
    /// list of config files to write to
    Env(Vec<String>),
}

pub(crate) fn write_config(
    config: RpcConfig,
    config_write_mode: Option<ConfigWriteMode>,
) -> Result<(), MescCliError> {
    let config_write_mode = match config_write_mode {
        Some(config_write_mode) => Some(config_write_mode),
        None => get_config_write_mode()?,
    };
    if config_write_mode.is_none() {
        println!(" No write mode selected");
        println!(" Not writing changes to disk");
    } else {
        match &config_write_mode {
            Some(write_mode) => match write_mode {
                ConfigWriteMode::Path(path) => {
                    mesc::write::write_config(config, path.clone())?;
                    println!(" {} {}", "config written to".bold(), path.to_string_lossy().green());
                }
                ConfigWriteMode::Env(_) => {
                    todo!("writing environment variables not supported yet")
                }
            },
            None => return Err(MescCliError::Error("could not obtain write mode".to_string())),
        };
    };
    Ok(())
}

pub(crate) fn get_config_write_mode() -> Result<Option<ConfigWriteMode>, MescCliError> {
    if let Ok(path) = mesc::load::get_config_path() {
        return Ok(Some(ConfigWriteMode::Path(path.into())));
    };

    let prompt = "How do you want to save the config?";
    let options = ["Save as path file", "Save as an environment variable"].to_vec();
    loop {
        let write_mode = match inquire::Select::new(prompt, options.clone()).prompt() {
            Ok("Save as path file") => {
                let prompt = "Where should mesc.json file be saved? (enter a directory path)";
                let parent = match inquire::Text::new(prompt).prompt() {
                    Ok(parent) => {
                        let parent = if parent.trim().is_empty() {
                            ".".to_string()
                        } else {
                            mesc::load::expand_path(parent)?
                        };
                        std::path::PathBuf::from(parent)
                    }
                    Err(InquireError::OperationCanceled) => return Ok(None),
                    Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                };
                let path: String = parent.join("mesc.json").to_string_lossy().to_string();
                Some(ConfigWriteMode::Path(path.into()))
            }
            Ok("Save an environment variable") => {
                println!("not supported yet");
                continue;
            }
            Err(InquireError::OperationCanceled) => return Ok(None),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        };
        return Ok(write_mode);
    }
}
