use std::fs;

pub mod events;

pub fn read_input(year: &i32, day: &i32) -> String {
    fs::read_to_string(get_input_path(year, day))
        .expect("Failed to read input file")
}

pub fn get_input_path(year: &i32, day: &i32) -> String {
    format!("events_data/year_{}/day_{:02}/input.txt", year, day)
}
