use crate::MescCliError;
use mesc::Endpoint;
use toolstr::ColumnFormatShorthand;

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
        let sorted_endpoints = sort_endpoints(endpoints);
        for endpoint in sorted_endpoints.into_iter() {
            names.push(endpoint.name.clone());
            networks.push(endpoint.chain_id_string());
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
        table.add_column("network", networks)?;
        format
            .add_column(ColumnFormatShorthand::new().name("network").font_style(description_style));
        table.add_column("url", urls)?;
        format.add_column(ColumnFormatShorthand::new().name("url").font_style(option_style));
        format.print(table)?;
    };

    Ok(())
}

pub(crate) fn print_defaults(config: &mesc::RpcConfig) -> Result<(), MescCliError> {
    let mut classes = Vec::new();
    let mut networks = Vec::new();
    let mut names = Vec::new();
    classes.push("global default");
    if let Some(default_endpoint) = mesc::get_default_endpoint(None)? {
        names.push(default_endpoint.name.clone());
        networks.push(default_endpoint.chain_id_string());
    }
    for (chain_id, name) in config.network_defaults.iter() {
        classes.push("network default");
        networks.push(chain_id.to_string());
        names.push(name.clone());
    }
    let format = toolstr::TableFormat::default();

    let mut title_style = crate::metadata::get_theme_font_style("title")?;
    title_style.bold();
    let metavar_style = crate::metadata::get_theme_font_style("metavar")?;
    let mut description_style = crate::metadata::get_theme_font_style("description")?;
    description_style.bold();
    let option_style = crate::metadata::get_theme_font_style("option")?;
    let _content_style = crate::metadata::get_theme_font_style("content")?;
    let comment_style = crate::metadata::get_theme_font_style("comment")?;

    let format = toolstr::TableFormat {
        // indent: 4,
        ..format
    };
    let mut format =
        format.border_font_style(comment_style.clone()).label_font_style(title_style.clone());
    let mut table = toolstr::Table::default();
    table.add_column("", classes)?;
    format.add_column(ColumnFormatShorthand::new().name("").font_style(option_style));
    table.add_column("network", networks)?;
    format.add_column(ColumnFormatShorthand::new().name("network").font_style(description_style));
    table.add_column("endpoint", names)?;
    format.add_column(ColumnFormatShorthand::new().name("endpoint").font_style(metavar_style));
    format.print(table)?;

    // if config.profiles.is_empty() {
    //     // println!();
    //     // println!();
    //     // println!("[none]");
    // } else {
    //     println!();
    //     println!();
    //     toolstr::print_header("Additional Profiles", &theme);
    //     for (name, _profile) in config.profiles.iter() {
    //         println!("- {}", name);
    //     }
    // };
    Ok(())
}
