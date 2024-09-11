use crate::MescCliError;
use mesc::{Endpoint, RpcConfig};
use toolstr::{Colorize, ColumnFormatShorthand};

pub(crate) fn print_endpoint_json(endpoint: Endpoint) {
    match serde_json::to_string(&endpoint) {
        Ok(as_str) => println!("{}", as_str),
        Err(_) => eprintln!("could not serialize endpoint"),
    }
}

pub(crate) fn print_endpoint_pretty(endpoint: Endpoint) {
    println!("Endpoint: {}", endpoint.name);
    println!("- url: {}", endpoint.url);
    println!(
        "- chain_id: {}",
        endpoint.chain_id.map_or("-".to_string(), |chain_id| chain_id.to_string())
    );
    println!("- metadata: {:?}", endpoint.endpoint_metadata);
}

fn sort_endpoints(endpoints: &[mesc::Endpoint]) -> Vec<mesc::Endpoint> {
    let mut endpoints: Vec<mesc::Endpoint> = endpoints.to_vec();
    endpoints.sort_by(|e1, e2| match (e1.chain_id.clone(), e2.chain_id.clone()) {
        (Some(c1), Some(c2)) => c1.cmp(&c2),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    endpoints
}

pub(crate) fn print_endpoints(
    endpoints: &[mesc::Endpoint],
    reveal: bool,
) -> Result<(), MescCliError> {
    if endpoints.is_empty() {
        println!("[none]")
    } else {
        let mut title_style = crate::metadata::get_theme_font_style("title")?;
        title_style.bold();
        let metavar_style = crate::metadata::get_theme_font_style("metavar")?;
        let mut description_style = crate::metadata::get_theme_font_style("description")?;
        description_style.bold();
        let option_style = crate::metadata::get_theme_font_style("option")?;
        let _content_style = crate::metadata::get_theme_font_style("content")?;
        let comment_style = crate::metadata::get_theme_font_style("comment")?;

        let mut names = Vec::new();
        let mut networks = Vec::new();
        let mut urls = Vec::new();
        let mut network_names = Vec::new();
        let sorted_endpoints = sort_endpoints(endpoints);

        let all_network_names = mesc::network_names::get_network_names();
        for endpoint in sorted_endpoints.into_iter() {
            names.push(endpoint.name.clone());
            networks.push(endpoint.chain_id_string());

            let network_name = endpoint
                .chain_id
                .as_ref()
                .and_then(|chain_id| all_network_names.get(&chain_id).map(ToString::to_string))
                .unwrap_or_else(|| endpoint.chain_id_string());
            network_names.push(network_name);
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
        let mut format =
            format.border_font_style(comment_style.clone()).label_font_style(title_style.clone());
        let mut table = toolstr::Table::default();
        table.add_column("endpoint", names)?;
        format.add_column(ColumnFormatShorthand::new().name("endpoint").font_style(metavar_style));
        table.add_column("network", network_names)?;
        format.add_column(
            ColumnFormatShorthand::new().name("network").font_style(description_style.clone()),
        );
        // table.add_column("chain id", networks)?;
        // format.add_column(
        //     ColumnFormatShorthand::new().name("chain id").font_style(description_style),
        // );
        table.add_column("url", urls)?;
        format.add_column(ColumnFormatShorthand::new().name("url").font_style(option_style));
        format.print(table)?;
    };

    Ok(())
}

pub(crate) fn print_defaults(config: &RpcConfig) -> Result<(), MescCliError> {
    let mut classes = Vec::new();
    let mut networks = Vec::new();
    let mut names = Vec::new();
    let mut profiles = Vec::new();

    // global default endpoint
    classes.push("global default");
    if let Some(default_endpoint_name) = &config.default_endpoint {
        if let Some(endpoint) = config.endpoints.get(default_endpoint_name) {
            names.push(default_endpoint_name.clone());
            networks.push(endpoint.chain_id_string());
        } else {
            names.push("[none]".into());
            networks.push("[none]".into());
        }
    } else {
        names.push("[none]".into());
        networks.push("[none]".into());
    }
    profiles.push("-".to_string());

    // global network defaults
    for (chain_id, name) in config.network_defaults.iter() {
        classes.push("network default");
        networks.push(chain_id.to_string());
        names.push(name.clone());
        profiles.push("-".to_string());
    }

    // profile defaults
    for (profile_name, profile) in config.profiles.clone().into_iter() {
        // profile default endpoint
        classes.push("profile global default");
        if let Some(default_endpoint_name) = profile.default_endpoint {
            if let Some(endpoint) = config.endpoints.get(&default_endpoint_name) {
                names.push(default_endpoint_name.clone());
                networks.push(endpoint.chain_id_string());
            } else {
                names.push("[none]".into());
                networks.push("[none]".into());
            }
        } else {
            names.push("[none]".into());
            networks.push("[none]".into());
        }
        profiles.push(profile_name.clone());

        // profile network default
        for (chain_id, name) in profile.network_defaults.iter() {
            classes.push("profile network default");
            networks.push(chain_id.to_string());
            names.push(name.clone());
            profiles.push(profile_name.clone());
        }
    }

    let mut title_style = crate::metadata::get_theme_font_style("title")?;
    title_style.bold();
    let metavar_style = crate::metadata::get_theme_font_style("metavar")?;
    let mut description_style = crate::metadata::get_theme_font_style("description")?;
    description_style.bold();
    let option_style = crate::metadata::get_theme_font_style("option")?;
    let _content_style = crate::metadata::get_theme_font_style("content")?;
    let comment_style = crate::metadata::get_theme_font_style("comment")?;

    let format = toolstr::TableFormat::default();
    let mut format =
        format.border_font_style(comment_style.clone()).label_font_style(title_style.clone());
    let mut table = toolstr::Table::default();
    table.add_column("", classes)?;
    format.add_column(ColumnFormatShorthand::new().name("").font_style(option_style.clone()));
    table.add_column("network", networks)?;
    format.add_column(ColumnFormatShorthand::new().name("network").font_style(description_style));
    table.add_column("endpoint", names)?;
    format.add_column(ColumnFormatShorthand::new().name("endpoint").font_style(metavar_style));
    table.add_column("profile", profiles)?;
    format.add_column(ColumnFormatShorthand::new().name("profile").font_style(option_style));
    format.print(table)?;

    Ok(())
}

pub(crate) fn print_environment_variables(indent: usize) {
    let indentation = " ".repeat(indent);
    let indentation2 = " ".repeat(indent + 4);
    let env_vars = ["MESC_MODE", "MESC_PATH", "MESC_ENV"];
    println!("{}Current environment variables:", indentation);
    for env_var in env_vars.iter() {
        match std::env::var(env_var) {
            Ok(value) => println!("{}{}: {}", indentation2, env_var.bold(), value.green()),
            Err(_) => println!("{}{}: {}", indentation2, env_var.bold(), "[not set]".green()),
        }
    }
    let overrides = [
        "MESC_NETWORK_NAMES",
        "MESC_NETWORK_DEFAULTS",
        "MESC_ENDPOINTS",
        "MESC_DEFAULT_ENDPOINT",
        "MESC_GLOBAL_METADATA",
        "MESC_ENDPOINT_METADATA",
        "MESC_PROFILES",
    ];
    println!("{}Current environment overrides:", indentation);
    for env_var in overrides.iter() {
        match std::env::var(env_var) {
            Ok(value) => println!("{}{}: {}", indentation2, env_var.bold(), value.green()),
            Err(_) => println!("{}{}: {}", indentation2, env_var.bold(), "[not set]".green()),
        }
    }
}
