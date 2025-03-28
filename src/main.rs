use std::collections::HashMap;
use advent_of_code::{events, read_input};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Event year to run
    #[arg(short, long)]
    year: Option<i32>,

    /// Day of the event to run
    #[arg(short, long)]
    day: Option<i32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("year: {:?}, day: {:?}", args.year, args.day);

    let input = read_input(&2015, &1);
    println!("{:?}", events::year_2015::day_01::solve(&input));

    // let input_line = String::new();
    // let bomb_dir = input_line.trim().to_string();
    // bomb_dir.chars().for_each(|dir| match dir {
    //     'U' => (),
    //     'D' => (),
    //     'L' => (),
    //     'R' => (),
    //     _ => (),
    // });
}
