use std::env;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    print!("{}", serde_json::json!([ "Hello, world!" ]).to_string());
}
