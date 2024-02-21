use std::io::{self, BufRead};

use patterns_rust::{pdn_snils::PdnSnils, snils::Snils};
fn main() {
    println!("New type pattern example");
    println!("Enter SNILS");
    let mut lines = io::stdin().lock().lines();
    loop {
        let pdsnils = lines
            .next()
            .expect("Failed to read from stdin")
            .expect("expected a valid UTF-8 string")
            .parse::<Snils>()
            .map(PdnSnils::from);
        match pdsnils {
            Ok(v) => {
                println!("SNILS is hidden: {}", v);
                println!("unmasking");
                let sv = v.show();
                println!("trying again: {}", sv)
            }
            Err(e) => println!("Error validating SNILS: {e}"),
        }
    }
}
