use std::io::{self, BufRead};

use patterns_rust::snils::Snils;
fn main() {
    println!("New type pattern example");
    println!("Enter SNILS");
    let s = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("Failed to read from stdin")
        .expect("expected a valid UTF-8 string");
    match s.parse::<Snils>() {
        Ok(good) => println!("validated {}", good),
        Err(e) => println!("Error validating SNILS: {e}"),
    }
}
