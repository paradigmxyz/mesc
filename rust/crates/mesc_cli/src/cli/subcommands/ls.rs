use crate::{print_endpoints, LsArgs, MescCliError};

pub(crate) fn ls_command(args: LsArgs) -> Result<(), MescCliError> {
    // get endpoints
    let mut query = mesc::EndpointQuery::new();
    if let Some(network) = args.network {
        query = query.chain_id(network)?;
    }
    if let Some(name) = args.name {
        query = query.name(name)?;
    }
    if let Some(url) = args.url {
        query = query.url(url)?;
    }
    let endpoints = mesc::find_endpoints(query)?;

    // check reveal
    let config = mesc::load::load_config_data()?;
    let reveal = if args.reveal {
        true
    } else {
        config.global_metadata.get("reveal") == Some(&serde_json::Value::Bool(true))
    };

    // output endpoints
    if args.json {
        println!("{}", serde_json::to_string_pretty(&endpoints)?);
        Ok(())
    } else {
        print_endpoints(&endpoints, reveal)
    }
}
