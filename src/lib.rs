use std::fs;

pub mod events;

pub fn read_input(year: &i32, day: &i32) -> String {
    fs::read_to_string(get_input_path(year, day))
        .expect("Failed to read input file")
        .trim_end()
        .to_string()
}

pub fn get_input_path(year: &i32, day: &i32) -> String {
    format!("events_data/year_{}/day_{:02}/input.txt", year, day)
}

#[cfg(test)]
mod tests {
    use crate::get_input_path;

    #[test]
    fn test_get_input_path() {
        assert_eq!("events_data/year_2015/day_14/input.txt", get_input_path(&2015, &14));
        assert_eq!("events_data/year_2000/day_01/input.txt", get_input_path(&2000, &1));
    }
}