use std::fs;

use stellar_core::syntax::{parse::parse, scan::scan};

pub fn run(filepath: &str) {
    let contents = fs::read_to_string(filepath).expect("Failed to read the file");

    let token_stream = scan(&contents).expect("Error scanning");

    println!("{:?}", parse(token_stream));
}
