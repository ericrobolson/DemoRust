pub mod backends;
pub mod environment;
pub mod error;
pub mod intermediate_representation;
pub mod location;
pub mod parser;
pub mod tokenizer;

use benchy::Benchy;
use std::{fs, path::PathBuf};

fn main() {
    {
        Benchy::time("read_file");

        let path: PathBuf = "../test.egg".into();

        let contents = fs::read_to_string(&path).expect("Something went wrong reading the file");
        println!("{}", contents);

        println!("\n\n\n");

        let tokens = tokenizer::Tokenizer::tokenize(&contents, path).unwrap();

        let nodes = parser::Parser::parse(tokens).unwrap();

        println!("{:#?}", nodes);
    }
}

fn save_benchmarks() {
    use chrono::Utc;

    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();

    println!("Current timestamp is {}", timestamp);

    Benchy::save(format!("_benchmarks/run_{}.txt", timestamp).as_str());
}
