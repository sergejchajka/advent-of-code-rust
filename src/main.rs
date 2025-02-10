use reqwest;
use std::fs;
use std::io::Read;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let data = fs::read_to_string("./data/input.txt").unwrap();

    let mut floor = 0;
    data.chars().for_each(|c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
    });

    println!("Result: {}", floor); // result 138
}
