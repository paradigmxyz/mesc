use crate::{MescCliError, UrlArgs};

pub(crate) fn url_command(args: UrlArgs) -> Result<(), MescCliError> {
    let endpoint = match args.query {
        Some(query) => mesc::parse_user_query(query.as_str(), args.profile.as_deref()),
        None => mesc::get_default_endpoint(args.profile.as_deref()),
    };
    match endpoint {
        Ok(Some(endpoint)) => println!("{}", endpoint.url),
        Ok(None) => {}
        Err(_) => eprintln!("could not load RPC config"),
    };
    Ok(())
}
