pub fn run() {
    // Print to console, and hey, that's a comment!
    println!("Hello from print.rs");

    // Basic Formatting
    println!("{} is from {}", "Sazz", "Brazil");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Sazz", "Brazil", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {game}",
        name = "Augusto",
        game = "CSGO"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholders for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
