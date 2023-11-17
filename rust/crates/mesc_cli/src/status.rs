pub(crate) fn print_status() {
    if mesc::is_mesc_enabled() {
        println!("MESC is enabled");
    } else {
        println!("MESC not enabled");
        return
    }

    // print configuration mode
    // if in path mode, print path
    //
}
