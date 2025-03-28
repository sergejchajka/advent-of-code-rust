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

    let registry = get_registry();

    let solutions: Vec<_> = registry.iter().filter(|(key, _)| 
        args.year.is_none_or(|y| y == key.0) && args.day.is_none_or(|d| d == key.1)
    ).collect();

    solutions.iter().for_each(|((year, day), func)| {
        let input = read_input(year, day);
        let result = func(&input);
        println!("y{:?} d{:02?}: {:?}", year, day, result);
    })
}

fn get_registry() -> HashMap<(i32, i32), fn(&str) -> (i32, i32)> {
    let mut map = HashMap::new();
    map.insert((2015, 1), events::year_2015::day_01::solve as fn(&str) -> (i32, i32));
    map.insert((2015, 2), events::year_2015::day_02::solve);
    map
}
