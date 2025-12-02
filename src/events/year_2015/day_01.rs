// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {

    let part_1_result = part_1(input);
    let part_2_result = part_2(input);

    (part_1_result, part_2_result)
}

fn part_1(input: &str) -> i64 {
    let mut floor = 0;

    input.chars().for_each(|c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
    });

    floor
}

fn part_2(input: &str) -> i64 {
    let mut basement: i64 = -1;
    let mut floor = 0;

    for (index, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
        if floor == -1 {
            basement = (index + 1) as i64;
            break;
        }
    }

    basement
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&2015, &1);
        assert_eq!(part_1(&input), 138);
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&2015, &1);
        assert_eq!(part_2(&input), 1771);
    }
}
