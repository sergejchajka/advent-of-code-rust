// TODO implement common function interface
pub fn solve(input: &str) -> (i32, i32) {

    let part_1_result = part_1(input);
    let part_2_result = part_2(input);

    (part_1_result, part_2_result)
}

fn part_1(input: &str) -> i32 {
    let mut value: i32 = 1;
    loop {
        let test_value = format!("{}{:?}", input, value);
        let value_digest = md5::compute(test_value.as_bytes());
        let value_digest = format!("{:x}", value_digest);

        if value_digest.starts_with("00000") || value > 1_000_000_000 {
            // println!("{}: {}", test_value, value_digest);
            break;
        }
        value += 1;
    }
    value
}

fn part_2(input: &str) -> i32 {
    let mut value: i32 = 1;
    loop {
        let test_value = format!("{}{:?}", input, value);
        let value_digest = md5::compute(test_value.as_bytes());
        let value_digest = format!("{:x}", value_digest);

        if value_digest.starts_with("000000") || value > 1_000_000_000 {
            // println!("{}: {}", test_value, value_digest);
            break;
        }
        value += 1;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&2015, &4);
        assert_eq!(part_1(&input), 117946);
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(part_2("abcdef"), 6742839);
        assert_eq!(part_2("pqrstuv"), 5714438);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&2015, &4);
        assert_eq!(part_2(&input), 3938038);
    }
}
