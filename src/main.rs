use reqwest;
use std::fs;
use std::io::Read;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let data = fs::read_to_string("input/input-1.txt").unwrap();

    println!("Data: {}", data);
}
