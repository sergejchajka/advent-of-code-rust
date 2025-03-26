use advent_of_code::events::year_2015::day_01::solve;
use advent_of_code::read_input;

#[tokio::main]
async fn main() {
    let input = read_input(&2015, &1);
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
