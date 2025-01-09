use reqwest;
use std::fs;
use std::io::Read;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let data = fs::read_to_string("../data/input.txt").unwrap();

    println!("Data: {}", data);
}
