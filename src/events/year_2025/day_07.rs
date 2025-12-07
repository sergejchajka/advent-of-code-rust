use std::ops::RangeInclusive;
use crate::events::year_2025::matrix::{Matrix, Point};

// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, _debug: bool) -> i64 {
    let mut total_splits= 0;
    let mut matrix = Matrix::from_input(input);

    for y in 1..matrix.height {
        for x in 0..matrix.width {
            let item_above = matrix.get(x, y - 1);
            let current_item = matrix.get(x, y);
            match (item_above, current_item) {
                ('S', '.') |  ('|', '.') => matrix.replace(x, y, '|'),
                ('|', '^') => {
                    total_splits += 1;
                    matrix.replace(x-1, y, '|');
                    matrix.replace(x+1, y, '|');
                },
                _ => ()
            }
        }
        if _debug {
            matrix.print();
            println!("total: {}", total_splits)
        }
    }

    total_splits
}

fn part_2(input: &str, _debug: bool) -> i64 {
    let mut matrix = Matrix::from_input(input);

    let mut pipe_matrix: Vec<Vec<i64>> = vec![vec![0; matrix.width]; matrix.height];

    for y in 1..matrix.height {
        for x in 0..matrix.width {
            let item_above = matrix.get(x, y - 1);
            let current_item = matrix.get(x, y);
            match (item_above, current_item) {
                ('S', '.') |  ('|', '.') => {
                    matrix.replace(x, y, '|');
                    pipe_matrix[y][x] += if item_above == 'S' { 1 } else { pipe_matrix[y-1][x] };
                },
                ('|', '^') => {
                    matrix.replace(x-1, y, '|');
                    pipe_matrix[y][x-1] += pipe_matrix[y-1][x];

                    matrix.replace(x+1, y, '|');
                    pipe_matrix[y][x+1] = pipe_matrix[y-1][x] + pipe_matrix[y-1][x+1];
                },
                _ => ()
            }
        }
        if _debug {
            matrix.print();
            let str = pipe_matrix[y].iter().map(|n| n.to_string() + ", ").collect::<String>();
            println!("pipes: {}", str);
            println!();
        }
    }

    pipe_matrix.last().unwrap().iter().sum()
}
// ...|.|.|||.|...
// ..|^|^|||^|^|..
// pipes: 0, 0, 0, 1, 0, 4, 0, 3, 3, 1, 0, 1, 0, 0, 0,
// pipes: 0, 0, 1, 0, 5, 0, 4, 3, 7, ^, 2, ^, 1, 0, 0,
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, read_input_raw};

    const YEAR: i32 = 2025;
    const DAY: i32 = 7;

    #[test]
    fn test_part_1_sample_input() {
        let sample_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(part_1(sample_input, false), 21);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 1524);
    }

    #[test]
    fn test_part_2_sample_input() {
        let sample_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(part_2(sample_input, true), 40);
    }

    #[test]
    fn test_part_2_task_input() {

        let input = read_input_raw(&YEAR, &DAY);
        assert_eq!(part_2(&input, false), 32982105837605);
    }
}

#[derive(Debug)]
pub struct MatrixDig {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<i64>>,
}

impl MatrixDig {

    // pub fn from_input(input: &str) -> Self {
    //     let mut data: Vec<Vec<char>> = Vec::new();
    //     input.lines().for_each( |line| data.push(line.chars().collect()));
    //     let width = data[0].len();
    //     let height = data.len();
    //
    //     Self {
    //         width,
    //         height,
    //         data,
    //     }
    // }

    pub fn get(&self, x: usize, y: usize) -> i64 {
        self.data[y][x]
    }

    pub(crate) fn get_line(&self, y: usize) -> Vec<i64> {
        self.data[y].clone()
    }

    pub fn replace(&mut self, x: usize, y: usize, item: i64) {
        self.data[y][x] = item
    }

    // pub fn count_adjacent_items_count(&self, x: usize, y: usize, item: char) -> usize {
    //     let mut count = 0;
    //     for dx in -1..=1 {
    //         for dy in -1..=1 {
    //             // skip itself
    //             if dx == 0 && dy == 0 { continue; }
    //
    //             let nx = x as i32 + dx;
    //             let ny = y as i32 + dy;
    //
    //             // skip out of bounds
    //             if nx < 0 || nx >= self.width as i32 { continue; }
    //             // skip out of bounds
    //             if ny < 0 || ny >= self.height as i32 { continue; }
    //
    //             count += if self.data[ny as usize][nx as usize] == item { 1 } else { 0 };
    //         }
    //     }
    //     count
    // }

    pub fn print(&self) {
        let result = self.data.iter()
            .map(|row| row.iter().cloned().map(|n| n.to_string()).collect::<String>() + "\n")
            .collect::<String>();
        println!("{}", result);
    }
}
