// TODO implement common function interface
pub fn solve(input: &str) -> (i32, i32) {

    let part_1_result = part_1(input);
    let part_2_result = part_2(input);

    (part_1_result, part_2_result)
}

fn part_1(_input: &str) -> i32 {
    todo!()
}

fn part_2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(0, part_1("turn on 0,0 through 999,999"));
        assert_eq!(0, part_1("toggle 0,0 through 999,0"));
        assert_eq!(0, part_1("turn off 499,499 through 500,500"));
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&2015, &1);
        assert_eq!(0, part_1(&input));
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(1, part_2(")"));
        assert_eq!(5, part_2("()())"));
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&2015, &1);
        assert_eq!(0, part_2(&input));
    }
}
