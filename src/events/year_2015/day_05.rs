// TODO implement common function interface
pub fn solve(input: &str) -> (i32, i32) {

    let part_1_result = part_1(input);
    let part_2_result = part_2(input);

    (part_1_result, part_2_result)
}

fn part_1(input: &str) -> i32 {
    fn has_three_vowels(line: &str) -> bool {
        let chars: Vec<_> = "aeiou".chars().collect();
        let count = line.chars().filter(|c| chars.contains(c)).count();
        count >= 3
    }
    
    fn has_consecutive_letters(line: &str) -> bool {
        let chars: Vec<_> = line.chars().collect();
        let mut last_char = '-';
        for c in chars {
            if last_char == c {
                 return true;
            } else { 
                last_char = c;
            }
        }
        false
    }
    
    fn has_no_exclusive_letters(line: &str) -> bool {
        let vec = ["ab", "cd", "pq", "xy"];
        for pair in vec {
            if line.contains(pair) {
                return false;
            }
        }
        true
    }
    
    let result = input.lines().filter(|line| {
        return has_three_vowels(line) && has_consecutive_letters(line) && has_no_exclusive_letters(line);
    }).count() as i32;
    result
}

fn part_2(input: &str) -> i32 {
    let result = -1;

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_1_task_samples() {
        assert_eq!(1, part_1("ugknbfddgicrmopn"));
        assert_eq!(1, part_1("aaa"));
        assert_eq!(0, part_1("jchzalrnumimnmhp"));
        assert_eq!(0, part_1("haegwjzuvuyypxyu"));
        assert_eq!(0, part_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&2015, &5);
        assert_eq!(238, part_1(&input));
    }

    #[test]
    fn test_part_2_task_samples() {
        assert_eq!(0, part_2("abcdef"));
        assert_eq!(0, part_2("pqrstuv"));
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&2015, &5);
        assert_eq!(0, part_2(&input));
    }
}
