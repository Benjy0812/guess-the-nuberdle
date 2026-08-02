use std::io::{self, Write};

fn main() {
    println!("Welcome to numberdle.");
    println!("A game like wordle but insted you guess numbers.");
    println!("1 Easy: (1-5)");
    println!("2 Medium: (1-10)");
    println!("3 hard: (1-15)");
    print!("Please chose difficulty:");
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut diff = String::new();
    io::stdin()
        .read_line(&mut diff)
        .expect("Failed to read input.");
}
