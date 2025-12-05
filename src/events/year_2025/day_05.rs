use std::ops::RangeInclusive;

// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, _debug: bool) -> i64 {
    let mut fresh_ids_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut ids_to_check: Vec<usize> = Vec::new();

    input.lines()
        .for_each(|line| {
            if line.contains('-') {
                let mut split = line.split('-');
                let left = split.next().unwrap().parse::<usize>().unwrap();
                let right = split.next().unwrap().parse::<usize>().unwrap();
                fresh_ids_ranges.push(left..=right);
            } else if !line.is_empty() {
                ids_to_check.push(line.parse::<usize>().unwrap());
            }
        });
    ids_to_check.iter()
        .filter(|id| fresh_ids_ranges.iter().any(|range| range.contains(id)))
        .count() as i64
}

fn part_2(input: &str, _debug: bool) -> i64 {
    let mut ranges = input.lines()
        .filter(|line| line.contains('-'))
        .map(|line| {
            let mut split = line.split('-');
            let left = split.next().unwrap().parse::<i64>().unwrap();
            let right = split.next().unwrap().parse::<i64>().unwrap();
            left..=right
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    ranges.sort_by( |a, b| a.start().cmp(b.start()));

    let folded_ranges = ranges.iter()
        .fold(<Vec<RangeInclusive<i64>>>::new(), |mut acc, next_range| {
            let last = acc.last();
            if last.is_none() { acc.push(next_range.clone())}
            else {
                let range = last.unwrap();
                if range.end() >= next_range.start() {
                    let index = acc.len() - 1;
                    acc[index] = range.start().clone()..=range.end().max(next_range.end()).clone()
                } else {
                    acc.push(next_range.clone())
                }
            }
            acc
        });

    folded_ranges.iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    const YEAR: i32 = 2025;
    const DAY: i32 = 5;

    #[test]
    fn test_part_1_sample_input() {
        let sample_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_1(sample_input, false), 3);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 681);
    }

    #[test]
    fn test_part_2_sample_input() {
        let sample_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_2(sample_input, false), 14);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_2(&input, true), 348820208020395);
    }
}
