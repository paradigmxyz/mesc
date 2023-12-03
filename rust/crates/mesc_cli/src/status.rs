pub(crate) fn print_status() {
    if mesc::is_mesc_enabled() {
        println!("MESC is enabled");
    } else {
        println!("MESC not enabled");
    }

    match mesc::load::get_config_mode() {
        Ok(mode) => println!("config mode: {:?}", mode),
        Err(e) => println!("{:?}", e),
    }

    // print configuration mode
    // if in path mode, print path
    let config = mesc::load::load_config_data();
    let config = match config {
        Err(e) => {
            println!("could not load config: {:?}", e);
            return
        },
        Ok(config) => config,
    };
    println!("config: {:?}", config)
}
