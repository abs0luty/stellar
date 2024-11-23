use std::fs;

use lasso::Rodeo;
use stellar_core::lang::scan::scan;

pub fn run(filepath: &str) {
    let contents = fs::read_to_string(filepath)
    .expect("Failed to read the file");

    let mut rodeo = Rodeo::new();
    println!("{:?}", scan(&contents, &mut rodeo));
}
