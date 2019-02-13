use std::env;
use serde_json;
use std::io::stdin;
use std::io::stdout;
use std::io::{ Write, Read };

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        println!("{}", command);
    }

    print!("{}", serde_json::json!([ "Quitted" ]).to_string());
}
