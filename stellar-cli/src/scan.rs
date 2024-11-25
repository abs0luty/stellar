use std::fs;

use stellar_core::syntax::scan::scan;

pub fn run(filepath: &str) {
    let contents = fs::read_to_string(filepath).expect("Failed to read the file");

    println!("{:?}", scan(&contents));
}
