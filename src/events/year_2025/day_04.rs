use crate::events::year_2025::day_04::matrix::{Matrix, Point};

pub mod matrix;

// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, _debug: bool) -> i64 {
    let matrix = Matrix::from_input(input);
    let mut count = 0;

    for x in 0..matrix.width {
        for y in 0..matrix.height {
            // skip empty cells
            if matrix.get(x, y) == '.' { continue; }

            if matrix.count_adjacent_items_count(x, y, '@') < 4 { count += 1; }
        }
    }

    count
}

fn part_2(input: &str, _debug: bool) -> i64 {
    let mut matrix = Matrix::from_input(input);
    let mut total_removed = 0;

    'remove_loop: loop {
        let mut rolls_to_remove: Vec<Point> = Vec::new();
        for x in 0..matrix.width {
            for y in 0..matrix.height {
                // skip empty cells
                if matrix.get(x, y) == '.' { continue; }

                if matrix.count_adjacent_items_count(x, y, '@') < 4 {
                    rolls_to_remove.push(Point { x, y });
                }
            }
        }

        if rolls_to_remove.len() == 0 { break 'remove_loop; }
        total_removed += rolls_to_remove.len();
        rolls_to_remove.iter().for_each(|p| matrix.replace(p.x, p.y, '.'));
    }

    total_removed as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    const YEAR: i32 = 2025;
    const DAY: i32 = 4;

    #[test]
    fn test_part_1_sample_input() {
        let sample_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_1(sample_input, false), 13);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 1547);
    }

    #[test]
    fn test_part_2_sample_input() {
        let sample_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_2(sample_input, false), 43);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_2(&input, true), 8948);
    }
}
