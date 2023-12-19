use crate::printing::{print_endpoint_json, print_endpoint_pretty};
use crate::{EndpointArgs, MescCliError};

pub(crate) fn endpoint_command(args: EndpointArgs) -> Result<(), MescCliError> {
    // get endpoint
    let endpoint = match (args.name, args.network, args.query) {
        (Some(name), _, _) => mesc::get_endpoint_by_name(name.as_str()).map(Some),
        (None, Some(_), Some(_)) => {
            return Err(MescCliError::InvalidInput(
                "specify either query or --network".to_string(),
            ))
        }
        (None, Some(network), None) => {
            mesc::get_endpoint_by_network(network.as_str(), args.profile.as_deref())
        }
        (None, None, Some(query)) => {
            mesc::parse_user_query(query.as_str(), args.profile.as_deref())
        }
        (None, None, None) => mesc::get_default_endpoint(args.profile.as_deref()),
    };

    // print endpoint
    match endpoint {
        Ok(Some(endpoint)) => {
            if args.json {
                print_endpoint_json(endpoint);
            } else {
                print_endpoint_pretty(endpoint);
            }
        }
        Ok(None) => {}
        Err(e) => eprintln!("could not load RPC config: {:?}", e),
    };
    Ok(())
}
