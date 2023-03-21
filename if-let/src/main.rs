fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Maximum is configured to be: {max}"),
        _ => (),
    }

    // instead of this syntax, if let can also be used to check for only one value of the enum.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Maximum: {max}");
    }
}
