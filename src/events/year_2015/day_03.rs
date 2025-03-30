use std::collections::HashSet;

// TODO implement common function interface
pub fn solve(input: &str) -> (i32, i32) {

    let part_1_result = part_1(input);
    let part_2_result = part_2(input);

    (part_1_result, part_2_result)
}

fn part_1(input: &str) -> i32 {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();

    let mut location = (0, 0); // (x, y)
    visited_houses.insert(location);

    input.chars().for_each(|direction| {
        location = match direction {
            '^' => (location.0, location.1 + 1),
            'v' => (location.0, location.1 - 1),
            '<' => (location.0 - 1, location.1),
            '>' => (location.0 + 1, location.1),
            _ => location
        };
        visited_houses.insert(location);
    });

    visited_houses.iter().count() as i32
}

fn part_2(input: &str) -> i32 {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();

    let mut santa_location = (0, 0); // (x, y)
    let mut robo_santa_location = (0, 0); // (x, y)
   
    visited_houses.insert(santa_location);
    visited_houses.insert(robo_santa_location);

    for (index, direction) in input.chars().enumerate() {

        let mut location = if index % 2 == 0 {
            santa_location
        } else {
            robo_santa_location
        };
        
        match direction {
            '^' => location.1 += 1,
            'v' => location.1 -= 1,
            '<' => location.0 -= 1,
            '>' => location.0 += 1,
            _ => {}
        };

        if index % 2 == 0 {
            santa_location = location
        } else {
            robo_santa_location = location       
        };
        
        visited_houses.insert(location.clone());
    };
    visited_houses.iter().count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&2015, &3);
        assert_eq!(part_1(&input), 2592);
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&2015, &3);
        assert_eq!(part_2(&input), 2360);
    }
}
