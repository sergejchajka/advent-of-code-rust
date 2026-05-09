use crate::events::year_2025::matrix::Matrix;
use crate::util::board::Board;
use crate::util::point_2d::Point2D;
use crate::util::point_3d::Point3D;

// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, false);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, _debug: bool) -> i64 {
    let points = input.lines()
        .map(|line| line.split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|points| {
            Point2D {
                y: points[1],
                x: points[0],
            }
        }).collect::<Vec<Point2D>>();


    let distances = calc_all_squares(&points);

    let (result, p1, p2)  = distances.last().unwrap();
    result.clone()
}

fn calc_all_squares<'a>(points: &'a [Point2D]) -> Vec<(i64, &'a Point2D, &'a Point2D)> {
    let mut distances = points.iter()
        .enumerate().
        map(|(i, p1)| {
            points.iter().enumerate()
                .filter_map(|(j, p2)| {
                    if i <= j { return None; }
                    let distance = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
                    Some((distance, p1, p2))
                })
                .collect::<Vec<(i64, &Point2D, &Point2D)>>()
        })
        .flatten()
        .collect::<Vec<(i64, &Point2D, &Point2D)>>();

    distances.sort_by(|p1, p2| { p1.0.partial_cmp(&p2.0).unwrap() });

    distances
}

fn part_2(input: &str, _debug: bool) -> i64 {
    let mut points = input.lines()
        .map(|line| line.split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|points| {
            Point2D {
                y: points[1],
                x: points[0],
            }
        }).collect::<Vec<Point2D>>();

    let distances = calc_all_squares(&points);

    let width = points.iter().max_by(| p1, p2| p1.x.cmp(&p2.x)).unwrap().x + 2;
    let height = points.iter().max_by(| p1, p2| p1.y.cmp(&p2.y)).unwrap().y + 2;
    let mut matrix = Matrix::new(width as usize, height as usize);
    points.windows(2).for_each(|pair| {
        let p1 = &pair[0];
        let p2 = &pair[1];
        matrix.replace(p1.x as usize, p1.y as usize, '#');
        matrix.replace(p2.x as usize, p2.y as usize, '#');

        if p1.x == p2.x {
            (p1.y.min(p2.y)+1..p1.y.max(p2.y)).for_each(|y| matrix.replace(p1.x as usize, y as usize, 'X'));
        } else {
            (p1.x.min(p2.x)+1..p1.x.max(p2.x)).for_each(|x| matrix.replace(x as usize, p1.y as usize, 'X'));
        }
    });
    matrix.print();
    let (result, p1, p2)  = distances.last().unwrap();

    result.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, read_input_raw};

    const YEAR: i32 = 2025;
    const DAY: i32 = 9;

    #[test]
    fn test_part_1_sample_input() {
        let sample_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        assert_eq!(part_1(sample_input, false), 50);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, false), 4771532800);
    }

    #[test]
    fn test_part_2_sample_input() {
        let sample_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        assert_eq!(part_2(sample_input, false), 24);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input_raw(&YEAR, &DAY);
        assert_eq!(part_2(&input, false), 6083499488);
    }
}
