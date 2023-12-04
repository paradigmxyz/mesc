use inquire::ui::{Attributes, Color, IndexPrefix, RenderConfig, StyleSheet, Styled};
use mesc::{Endpoint, MescError};

pub fn run_setup() -> Result<(), MescError> {
    inquire::set_global_render_config(get_render_config());

    if mesc::is_mesc_enabled() {
        let _config = mesc::load::load_config_data();
        println!("MESC is enabled");
        println!();
        setup_existing_config()
    } else {
        setup_new_config()
    }
}

fn setup_new_config() -> Result<(), MescError> {
    println!("MESC is not enabled, creating config now...");
    println!();
    println!("default endpoint URL?");
    println!();
    println!("default endpoint name?");
    Ok(())
}

fn setup_existing_config() -> Result<(), MescError> {
    let options = [
        "add new endpoint",
        "modify endpoint",
        "modify defaults",
        "exit",
    ]
    .to_vec();
    match inquire::Select::new("What do you want to do?", options).prompt() {
        Ok("add new endpoint") => add_endpoint()?,
        Ok("modify endpoint") => modify_endpoint()?,
        Ok(_) => todo!(),
        Err(_) => todo!(),
    };
    Ok(())
}

fn add_endpoint() -> Result<(), MescError> {
    let _endpoint = match inquire::Text::new("Endpoint URL?").prompt() {
        Ok(url) => {
            let default_name = "new_name";
            let input = inquire::Text::new("Endpoint name?").with_default(default_name);
            let name = match input.prompt() {
                Ok(choice) => choice,
                Err(_) => return Err(MescError::InvalidInput),
            };
            Endpoint {
                url,
                name,
                chain_id: None,
                endpoint_metadata: std::collections::HashMap::new(),
            }
        }
        Err(_) => return Err(MescError::InvalidInput),
    };
    Ok(())
}

fn modify_endpoint() -> Result<(), MescError> {
    let options = [
        "modify endpoint name",
        "modify endpoint url",
        "modify endpoint chain_id",
        "modify endpoint metadata",
        "delete endpoint",
    ]
    .to_vec();
    let _ans: Result<&str, inquire::InquireError> =
        inquire::Select::new("What do you want to do?", options).prompt();
    Ok(())
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
