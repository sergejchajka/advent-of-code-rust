use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::events::year_2025::Dial;

pub mod dial;

// TODO implement common function interface
pub fn solve(input: &str) -> (i32, i32) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, debug: bool) -> i32 {
    let mut zero_times = 0;
    let mut dial = Dial::default();

    if debug {
        println!(" - The dial starts by pointing at {}.", dial.position);
    }

    input.lines().for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        let (direction, step) = line.trim().split_at(1);
        dial.rotate(direction.into(), step.parse::<u32>().unwrap());


        if debug {
            println!(" - The dial is rotated {} to point at {}.", line, dial.position);
        }

        if dial.position == 0 { zero_times += 1 }
    });

    zero_times
}


#[warn(unused)]
fn part_2(input: &str, debug: bool) -> i32 {
    let mut dial = Dial::default();
    input.lines().for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        let (direction, step) = line.trim().split_at(1);
        dial.rotate(direction.into(), step.parse::<u32>().unwrap());

        if debug {
            println!(" - The dial is rotated {} to point at {}.", line, dial.position);
        }
    });

    dial.zero_crossed as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    const YEAR: i32 = 2025;
    const DAY: i32 = 1;

    const TEST_INPUT: &str = r#"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
        "#;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(part_1(TEST_INPUT, true), 3);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 1141);
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(part_2(TEST_INPUT, true), 6);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_2(&input, false), 6634);
    }
}
