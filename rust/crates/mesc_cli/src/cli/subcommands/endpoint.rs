use crate::printing::{print_endpoint_json, print_endpoint_pretty};
use crate::{EndpointArgs, MescCliError};

pub(crate) fn endpoint_command(args: EndpointArgs) -> Result<(), MescCliError> {
    let endpoint = match args.query {
        Some(query) => mesc::parse_user_query(query.as_str(), args.profile.as_deref()),
        None => mesc::get_default_endpoint(args.profile.as_deref()),
    };
    match endpoint {
        Ok(Some(endpoint)) => {
            if args.json {
                print_endpoint_json(endpoint);
            } else {
                print_endpoint_pretty(endpoint);
            }
        }
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    };
    Ok(())
}
