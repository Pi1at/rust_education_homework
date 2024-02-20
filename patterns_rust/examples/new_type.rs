use std::io::{self, BufRead};

use patterns_rust::snils::Snils;
fn main() {
    println!("New type pattern example");
    println!("Enter SNILS");
    let s = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("msg")
        .expect("msg");
    match s.parse::<Snils>() {
        Ok(good) => println!("validated {}", good),
        Err(e) => println!("{e}"),
    }
}
