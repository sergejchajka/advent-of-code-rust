// TODO implement common function interface
pub fn solve(input: &str) -> (i32, i32) {

    let part_1_result = part_1(input);
    let part_2_result = part_2(input);

    (part_1_result, part_2_result)
}

fn part_1(input: &str) -> i32 {
    let total_squares: i32 = input.lines().map(|line| {
        let parts = line.split("x").collect::<Vec<&str>>();

        let l: i32 = parts[0].parse().unwrap();
        let w: i32 = parts[1].parse().unwrap();
        let h: i32 = parts[2].parse().unwrap();

        let sides = [l*w, w*h, h*l];

        2 * sides.iter().sum::<i32>() + sides.iter().min().unwrap()
    }).sum();

    total_squares
}

fn part_2(input: &str) -> i32 {
    let total_squares: i32 = input.lines().map(|line| {
        let parts = line.split("x").collect::<Vec<&str>>();

        let l: i32 = parts[0].parse().unwrap();
        let w: i32 = parts[1].parse().unwrap();
        let h: i32 = parts[2].parse().unwrap();

        let mut sides = [l, w, h];
        sides.sort();

        (2 * (sides[0] + sides[1])) + (l*w*h)
    }).sum::<i32>();

    total_squares
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(part_1("2x3x4"), 58);
        assert_eq!(part_1("1x1x10"), 43);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&2015, &2);
        assert_eq!(part_1(&input), 1598415);
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(part_2("2x3x4"), 34);
        assert_eq!(part_2("1x1x10"), 14);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&2015, &2);
        assert_eq!(part_2(&input), 3812909);
    }
}
