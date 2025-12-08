// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, _debug: bool) -> i64 {
    input.split(',')
        .map(|range| check_ids_range_part_1(range))
        .flatten()
        .sum::<i64>()
}


#[warn(unused)]
fn part_2(input: &str, _debug: bool) -> i64 {
    input.split(',')
        .map(|range| check_ids_range_part_2(range))
        .flatten()
        .sum::<i64>()
}

fn check_ids_range_part_1(range: &str) -> Vec<i64> {
    let [from, to] = range
        .split('-')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[0..2] else { todo!() };

    (from..=to).filter_map(|id| {
        let id_str = id.to_string();

        if id_str.len() % 2 != 0 { return None }

        let (left, right) = id_str.split_at(id_str.len() / 2);

        return if left == right {
            Some(id)
        } else {
            None
        }
    }).collect::<Vec<i64>>()
}

fn check_ids_range_part_2(range: &str) -> Vec<i64> {
    let [from, to] = range
        .split('-')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[0..2] else { todo!() };

    (from..=to).filter_map(|id| {
        let id_str = id.to_string();

        let max_split = id_str.len() / 2;

        'chunk_iter: for chunk_size in 1..=max_split {
            if (id_str.len() % chunk_size) != 0 { continue 'chunk_iter }

            let chunks = id_str.as_bytes()
                .chunks(chunk_size)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect::<Vec<&str>>();

            if let Some((first, rest)) = chunks.split_first() {
                for chunk in rest {
                    if first != chunk { continue 'chunk_iter; }
                }
                return Some(id);
            } else {
                return None;
            }

            /* also works but slower
            let chunks = id_str.as_bytes()
                .chunks(chunk_size)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect::<HashSet<&str>>();

            if chunks.len() == 1 { return Some(id) } else { continue 'chunk_iter }
             */
        }
        // none if not returned before
        None
    }).collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    const YEAR: i32 = 2025;
    const DAY: i32 = 2;

    #[test]
    fn test_part_1_sample_input() {
        assert_eq!(check_ids_range_part_1("11-22"), [11, 22]);
        assert_eq!(check_ids_range_part_1("95-115"), [99]);
        assert_eq!(check_ids_range_part_1("998-1012"), [1010]);
        assert_eq!(check_ids_range_part_1("1188511880-1188511890"), [1188511885]);
        assert_eq!(check_ids_range_part_1("222220-222224"), [222222]);
        assert_eq!(check_ids_range_part_1("1698522-1698528"), []);
        assert_eq!(check_ids_range_part_1("446443-446449"), [446446]);
        assert_eq!(check_ids_range_part_1("38593856-38593862"), [38593859]);
    }

    #[test]
    fn test_part_1_sample_result() {
        let test_input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
        assert_eq!(part_1(test_input, true), 1227775554);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 35367539282);
    }

    #[test]
    fn test_part_2_sample_input() {
        assert_eq!(check_ids_range_part_2("11-22"), [11, 22]);
        assert_eq!(check_ids_range_part_2("95-115"), [99, 111]);
        assert_eq!(check_ids_range_part_2("998-1012"), [999, 1010]);
        assert_eq!(check_ids_range_part_2("1188511880-1188511890"), [1188511885]);
        assert_eq!(check_ids_range_part_2("222220-222224"), [222222]);
        assert_eq!(check_ids_range_part_2("1698522-1698528"), []);
        assert_eq!(check_ids_range_part_2("446443-446449"), [446446]);
        assert_eq!(check_ids_range_part_2("38593856-38593862"), [38593859]);
        assert_eq!(check_ids_range_part_2("565653-565659"), [565656]);
        assert_eq!(check_ids_range_part_2("824824821-824824827"), [824824824]);
        assert_eq!(check_ids_range_part_2("2121212118-2121212124"), [2121212121]);
    }

    #[test]
    fn test_part_2_sample_result() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_2(test_input, true), 4174379265);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_2(&input, false), 45814076230);
    }
}
