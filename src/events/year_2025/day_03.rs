// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, _debug: bool) -> i64 {
    input.lines()
        .map(|range| check_joltage_rating_part_1(range))
        .sum::<i64>()
}

fn check_joltage_rating_part_1(batteries_range: &str) -> i64 {
    let numbers = batteries_range.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut max_joltage_1 = 0;

    let len = numbers.len();

    let mut cutoff = 0;

    for i in 0..len - 1 {
        if max_joltage_1 < numbers[i] {
            max_joltage_1 = numbers[i];
            cutoff = i + 1;
        }
    }

    //shift value
    let mut max_joltage_2 = 0;

    for i in cutoff..len {
        if max_joltage_2 < numbers[i] {
            max_joltage_2 = numbers[i];
        }
    }

    (max_joltage_1 * 10 + max_joltage_2) as i64
}

#[warn(unused)]
fn part_2(input: &str, _debug: bool) -> i64 {
    input.lines()
        .map(|range| check_joltage_rating_part_2(range))
        .sum::<i64>()
}

fn check_joltage_rating_part_2(batteries_range: &str) -> i64 {
    let mut numbers = batteries_range.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut index = 0;

    'switch_loop: loop {
        if numbers[index] < numbers[index+1] {
            numbers.remove(index);
            if index > 0 { index -= 1;}
        } else {
            index += 1;
        }

        if numbers.len() == index + 1 {
            numbers = numbers.iter().take(12).cloned().collect();
            break 'switch_loop;
        }

        if numbers.len() == 12 {
            break 'switch_loop;
        }
    }

    // 'switch_loop: loop {
    //     if index == numbers.len() {
    //         index = 0;
    //         battery_to_switch = numbers.iter().min().unwrap().clone();
    //     }
    //
    //     if numbers[index] == battery_to_switch {
    //         numbers.remove(index);
    //         // check the same index in the next loop
    //     } else {
    //         index += 1;
    //     }
    //
    //     if numbers.len() == 12 {
    //         break 'switch_loop;
    //     }
    // }

    numbers.into_iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    const YEAR: i32 = 2025;
    const DAY: i32 = 3;

    #[test]
    fn test_part_1_sample_input() {
        assert_eq!(check_joltage_rating_part_1("987654321111111"), 98);
        assert_eq!(check_joltage_rating_part_1("811111111111119"), 89);
        assert_eq!(check_joltage_rating_part_1("234234234234278"), 78);
        assert_eq!(check_joltage_rating_part_1("818181911112111"), 92);
    }

    #[test]
    fn test_part_1_sample_result() {
        let test_input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_1(test_input, true), 357);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 16887);
    }

    #[test]
    fn test_part_2_sample_input() {
        assert_eq!(check_joltage_rating_part_2("987654321111111"), 987654321111);
        assert_eq!(check_joltage_rating_part_2("811111111111119"), 811111111119);
        assert_eq!(check_joltage_rating_part_2("234234234234278"), 434234234278);
        assert_eq!(check_joltage_rating_part_2("818181911112111"), 888911112111);
    }

    #[test]
    fn test_part_2_input_record() {
        assert_eq!(check_joltage_rating_part_2("2223223335223234342422322225224113422423142441542233322124236224232234222242262232142124444266221211"), 664466221211);
        assert_eq!(check_joltage_rating_part_2("2222212212234222414222212211222222224222222122313112221123221223222122222212121221222222122321323221"), 444433333332);
    }

    #[test]
    fn test_part_2_sample_result() {
        let test_input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_2(test_input, true), 3121910778619);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_2(&input, true), 167302518850275);
    }
}
