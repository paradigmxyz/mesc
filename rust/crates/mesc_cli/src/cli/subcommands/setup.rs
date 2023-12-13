use crate::{MescCliError, SetupArgs};
use inquire::ui::{Attributes, Color, IndexPrefix, RenderConfig, StyleSheet, Styled};
use mesc::{Endpoint, RpcConfig};

pub fn run_setup(args: SetupArgs) -> Result<(), MescCliError> {
    inquire::set_global_render_config(get_render_config());

    if args.editor {
        return edit_config_in_editor(args)
    }

    if mesc::is_mesc_enabled() {
        let config = mesc::load::load_config_data()?;
        println!("MESC is enabled");
        println!();
        modify_existing_config(config)
    } else {
        setup_new_config()
    }
}

fn edit_config_in_editor(args: SetupArgs) -> Result<(), MescCliError> {
    if let Some(path) = args.path {
        edit::edit_file(path)?
    } else if let Ok(mesc::ConfigMode::Path) = mesc::load::get_config_mode() {
        let path = mesc::load::get_config_path()?;
        edit::edit_file(path)?
    } else {
        return Err(MescCliError::Error("no file to edit".to_string()))
    };
    Ok(())
}

#[derive(Clone)]
enum ConfigWriteMode {
    /// file path of JSON config file
    Path(String),
    /// list of config files to write to
    Env(Vec<String>),
}

fn get_config_write_mode() -> ConfigWriteMode {
    todo!()
}

fn setup_new_config() -> Result<(), MescCliError> {
    println!("MESC is not enabled, creating config now...");
    println!();
    println!("default endpoint URL?");
    println!();
    println!("default endpoint name?");
    Ok(())
}

fn modify_existing_config(config: RpcConfig) -> Result<(), MescCliError> {
    let options = [
        "Add new endpoint",
        "Modify endpoint",
        "Modify defaults",
        "Exit",
    ]
    .to_vec();
    let mut old_config = config.clone();
    let config_write_mode = get_config_write_mode();
    loop {
        let new_config =
            match inquire::Select::new("What do you want to do?", options.clone()).prompt()? {
                "Add new endpoint" => add_endpoint(old_config.clone())?,
                "Modify endpoint" => modify_endpoint(old_config.clone())?,
                "Modify defaults" => modify_defaults(old_config.clone())?,
                "Exit" => return Ok(()),
                _ => return Err(MescCliError::InvalidInput),
            };

        if serde_json::to_string(&new_config)? != serde_json::to_string(&old_config)? {
            match inquire::Confirm::new("Save changes to file?").prompt() {
                Ok(true) => write_config(&new_config, config_write_mode.clone())?,
                Ok(false) => {}
                Err(_) => return Err(MescCliError::InvalidInput),
            }
        }

        old_config = new_config;
    }
}

fn write_config(
    _config: &RpcConfig,
    _config_write_mode: ConfigWriteMode,
) -> Result<(), MescCliError> {
    todo!()
}

fn add_endpoint(config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    let _endpoint = match inquire::Text::new("Endpoint URL?").prompt() {
        Ok(url) => {
            let default_name = "new_name";
            let input = inquire::Text::new("Endpoint name?").with_default(default_name);
            let name = match input.prompt() {
                Ok(choice) => choice,
                Err(_) => return Err(MescCliError::InvalidInput),
            };
            Endpoint {
                url,
                name,
                chain_id: None,
                endpoint_metadata: std::collections::HashMap::new(),
            }
        }
        Err(_) => return Err(MescCliError::InvalidInput),
    };
    Ok(config)
}

fn modify_endpoint(config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    // select endpoint
    let options: Vec<String> = config.endpoints.clone().into_keys().collect();
    let endpoint_name = inquire::Select::new("Which endpoint to modify?", options).prompt()?;

    // perform modifications
    let options = [
        "Modify endpoint name",
        "Modify endpoint url",
        "Modify endpoint chain_id",
        "Modify endpoint metadata",
        "Delete endpoint",
        "Done",
    ]
    .to_vec();
    let mut message = "What do you want to do?";
    let mut new_config = match inquire::Select::new(message, options.clone()).prompt() {
        Ok("Done") => return Ok(config),
        Ok("Delete Endpoint") => {
            let mut new_endpoints = config.endpoints.clone();
            new_endpoints.remove(&endpoint_name);
            let new_config = RpcConfig {
                endpoints: new_endpoints,
                ..config
            };
            return Ok(new_config);
        }
        Ok(_) => config,
        Err(_) => return Err(MescCliError::InvalidInput),
    };
    message = "Any other modifications?";
    loop {
        new_config = match inquire::Select::new(message, options.clone()).prompt() {
            Ok("Done") => return Ok(new_config),
            Ok("Delete Endpoint") => {
                let mut new_endpoints = new_config.endpoints.clone();
                new_endpoints.remove(&endpoint_name);
                let new_config = RpcConfig {
                    endpoints: new_endpoints,
                    ..new_config.clone()
                };
                return Ok(new_config);
            }
            Ok(_) => new_config,
            Err(_) => return Err(MescCliError::InvalidInput),
        };
    }
}

fn modify_defaults(config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    let options = [
        "Set default endpoint",
        "Set default endpoint for network",
        "Add new profile",
        "Modify existing profile",
    ]
    .to_vec();

    match inquire::Select::new("What do you want to do?", options).prompt()? {
        "Set default endpoint" => set_default_endpoint(config),
        "Set default endpoint for network" => set_default_endpoint_for_network(config),
        "Add new profile" => add_new_profile(config),
        "Modify existing profile" => modify_existing_profile(config),
        _ => Err(MescCliError::InvalidInput),
    }
}

fn set_default_endpoint(_config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    todo!()
}

fn set_default_endpoint_for_network(_config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    todo!()
}

fn add_new_profile(_config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    todo!()
}

fn modify_existing_profile(_config: RpcConfig) -> Result<RpcConfig, MescCliError> {
    todo!()
}

fn get_render_config() -> RenderConfig {
    let highlight_color = Color::DarkGreen;

    let mut render_config = RenderConfig::default();
    render_config.prompt = StyleSheet::new().with_attr(Attributes::BOLD);
    render_config.prompt_prefix = Styled::new("").with_fg(Color::LightRed);
    render_config.answered_prompt_prefix = Styled::new("").with_fg(Color::LightRed);
    render_config.placeholder = StyleSheet::new().with_fg(Color::LightRed);
    render_config.selected_option = Some(StyleSheet::new().with_fg(highlight_color));
    render_config.highlighted_option_prefix = Styled::new("→").with_fg(highlight_color);
    render_config.selected_checkbox = Styled::new("☑").with_fg(highlight_color);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");
    render_config.option_index_prefix = IndexPrefix::Simple;
    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));
    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(highlight_color);
    let grey = Color::Rgb {
        r: 100,
        g: 100,
        b: 100,
    };
    render_config.help_message = StyleSheet::new()
        .with_fg(grey)
        .with_attr(Attributes::ITALIC);

    render_config
}
