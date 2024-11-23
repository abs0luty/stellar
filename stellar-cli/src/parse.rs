use std::fs;

use lasso::Rodeo;
use stellar_core::lang::{parse::parse, scan::scan};

pub fn run(filepath: &str) {
    let contents = fs::read_to_string(filepath).expect("Failed to read the file");

    let mut rodeo = Rodeo::new();
    let token_stream = scan(&contents, &mut rodeo).expect("Error scanning");

    println!("{:?}", parse(token_stream));
}
