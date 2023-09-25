fn main() {
    // if we want one arm and ignore any other arm value we could...
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // instead we can use if-let syntactic sugar
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else { // it even has an else clause :)
        println!("No maximum configuration :(")
    }
}
