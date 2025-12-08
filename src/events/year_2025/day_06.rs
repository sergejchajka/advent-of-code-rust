// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn parse_line_part_1(line: &str) -> Vec<&str> {
    line.split(' ')
        .filter(|s| { !s.is_empty() })
        .collect()
}

fn part_1(input: &str, _debug: bool) -> i64 {
    let lines = input.lines().map(|line| parse_line_part_1(line)).collect::<Vec<Vec<&str>>>();
    let (operations, numbers) = lines.split_last().unwrap();

    operations.iter()
        .map(|c| { c.parse::<char>().unwrap() })
        .enumerate()
        .map(|(index, operation)| {
            match operation {
                '*' => numbers.iter()
                    .map(|line| line[index])
                    .fold(1, |acc, next| acc * next.parse::<i64>().unwrap()),
                '+' => numbers.iter()
                    .map(|line| line[index].parse::<i64>().unwrap())
                    .sum(),
                _ => panic!("Unknown operation"),
            }
        }).sum::<i64>()
}

fn part_2(input: &str, _debug: bool) -> i64 {
    let lines = input.lines().into_iter().collect::<Vec<&str>>();
    let (operations, numbers) = lines.split_last().unwrap();
    let mut total = 0 as i64;
    let mut last_operation  = ' ';
    let mut tmp_results:Vec<i64> = Vec::new();
    operations.chars().into_iter()
        .enumerate()
        .for_each(|(index, operation)| {
            if operation != ' ' {
                last_operation = operation;
            }

            let str = numbers.iter()
                .map(|line| line.chars().nth(index).unwrap_or(' '))
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            let is_the_end = operations.len() == index + 1;
            if !str.is_empty() || is_the_end {
                tmp_results.push(String::from_iter(str.clone()).parse::<i64>().unwrap())
            }
            if str.is_empty() || is_the_end {
                total += match last_operation {
                    '*' => tmp_results.iter()
                        .fold(1, |acc, next| acc * next),
                    '+' => tmp_results.iter().sum(),
                    _ => panic!("Unknown operation"),
                };
                tmp_results.clear();
            }
        });

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, read_input_raw};

    const YEAR: i32 = 2025;
    const DAY: i32 = 6;

    #[test]
    fn test_part_1_sample_input() {
        let sample_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

        assert_eq!(part_1(sample_input, false), 4277556);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 6371789547734);
    }

    #[test]
    fn test_part_2_sample_input() {
        let sample_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

        assert_eq!(part_2(sample_input, false), 3263827);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input_raw(&YEAR, &DAY);
        assert_eq!(part_2(&input, true), 11419862653216);
    }
}
