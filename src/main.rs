use advent_of_code::year_2015;

#[tokio::main]
async fn main() {
    let solver = year_2015::day_1::part_1::solve;
    let input = include_str!("year_2015/day_1/part_1_input.txt");
    println!("{}", solver(&input));
}
