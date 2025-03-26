use advent_of_code::events;
use advent_of_code::events::year_2015::day_01::solve;

#[tokio::main]
async fn main() {
    // let solver = events::year_2015::part_1::solve;
    let input = include_str!("events/year_2015/part_1_input.txt");
    
    println!("{:?}", solve(&input));

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
