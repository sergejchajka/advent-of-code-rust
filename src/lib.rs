use std::fs;

pub mod events;

pub fn read_input(year: &i32, day: &i32) -> String {
    fs::read_to_string(format!("inputs/year_{}/day_{:02}.txt", year, day))
        .expect("Failed to read input file")
}