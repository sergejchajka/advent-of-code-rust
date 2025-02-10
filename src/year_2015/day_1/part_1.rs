pub fn solve(input: &str) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_samples() {
        assert_eq!(solve("(())"), 0);
        assert_eq!(solve("()()"), 0);
        assert_eq!(solve("((("), 3);
        assert_eq!(solve("(()(()("), 3);
        assert_eq!(solve("))((((("), 3);
        assert_eq!(solve("())"), -1);
        assert_eq!(solve("))("), -1);
        assert_eq!(solve(")))"), -3);
        assert_eq!(solve(")())())"), -3);
    }

    #[test]
    fn test_task_input() {
        let input = include_str!("./part_1_input.txt");
        assert_eq!(solve(input), 138);
    }
}