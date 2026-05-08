use crate::util::board::Board;
use crate::util::point_3d::Point3D;

// TODO implement common function interface
pub fn solve(input: &str) -> (i64, i64) {
    let part_1_result = part_1(input, 1000);
    let part_2_result = part_2(input, false);

    (part_1_result, part_2_result)
}

fn part_1(input: &str, connections: usize) -> i64 {
    let points = input.lines()
        .map(|line| line.split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|points| {
            Point3D {
                x: points[0],
                y: points[1],
                z: points[2],
            }
        }).collect::<Vec<Point3D>>();

    let mut board = Board::new(points.clone());

    let distances = calc_all_distances(&points);

    let closest_connections = distances.iter().take(connections).cloned().collect::<Vec<(f64, &Point3D, &Point3D)>>();

    closest_connections.iter().for_each(|(_, p1, p2)| {
        board.join(p1, p2)
    });

    let mut sizes = board.circuits.iter().map(|circuit| circuit.len()).collect::<Vec<usize>>();
    sizes.sort();

    let result = sizes.iter().rev().take(3)
        .map(|&size| size)
        .reduce(|acc, next| acc * next).unwrap();

    result.clone() as i64
}

fn calc_all_distances<'a>(points: &'a [Point3D]) -> Vec<(f64, &'a Point3D, &'a Point3D)> {
    let mut distances = points.iter()
        .enumerate().
        map(|(i, point)| {
            points.iter().enumerate()
                .filter_map(|(j, other)| {
                    if i <= j { return None; }
                    let distance = point.euclidean_distance(other);
                    Some((distance, point, other))
                })
                .collect::<Vec<(f64, &Point3D, &Point3D)>>()
        })
        .flatten()
        .collect::<Vec<(f64, &Point3D, &Point3D)>>();

    distances.sort_by(|p1, p2| { p1.0.partial_cmp(&p2.0).unwrap() });

    distances
}

fn part_2(input: &str, _debug: bool) -> i64 {
    let points = input.lines()
        .map(|line| line.split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|points| {
            Point3D {
                x: points[0],
                y: points[1],
                z: points[2],
            }
        }).collect::<Vec<Point3D>>();

    let mut board = Board::new(points.clone());

    let mut closest_connections = calc_all_distances(&points);

    closest_connections.reverse();

    loop {
        let next = closest_connections.pop();
        if next.is_none() { panic!("No more connections"); }

        let (_, p1, p2) = next.unwrap();

        // println!("Connecting {} and {}", p1, p2);
        board.join(p1, p2);

        if board.circuits[0].len() == points.len() {
            return p1.x * p2.x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, read_input_raw};

    const YEAR: i32 = 2025;
    const DAY: i32 = 8;

    #[test]
    fn test_part_1_sample_input() {
        let sample_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(part_1(sample_input, 10), 40);
    }

    #[test]
    fn test_part_1_task_input() {
        let input = read_input(&YEAR, &DAY);
        assert_eq!(part_1(&input, 1000), 90036);
    }

    #[test]
    fn test_part_2_sample_input() {
        let sample_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(part_2(sample_input, true), 25272);
    }

    #[test]
    fn test_part_2_task_input() {
        let input = read_input_raw(&YEAR, &DAY);
        assert_eq!(part_2(&input, false), 6083499488);
    }
}

#[derive(Debug)]
pub struct MatrixDig {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<i64>>,
}

impl MatrixDig {

    // pub fn from_input(input: &str) -> Self {
    //     let mut data: Vec<Vec<char>> = Vec::new();
    //     input.lines().for_each( |line| data.push(line.chars().collect()));
    //     let width = data[0].len();
    //     let height = data.len();
    //
    //     Self {
    //         width,
    //         height,
    //         data,
    //     }
    // }

    pub fn get(&self, x: usize, y: usize) -> i64 {
        self.data[y][x]
    }

    // pub(crate) fn get_line(&self, y: usize) -> Vec<i64> {
    //     self.data[y].clone()
    // }

    pub fn replace(&mut self, x: usize, y: usize, item: i64) {
        self.data[y][x] = item
    }

    // pub fn count_adjacent_items_count(&self, x: usize, y: usize, item: char) -> usize {
    //     let mut count = 0;
    //     for dx in -1..=1 {
    //         for dy in -1..=1 {
    //             // skip itself
    //             if dx == 0 && dy == 0 { continue; }
    //
    //             let nx = x as i32 + dx;
    //             let ny = y as i32 + dy;
    //
    //             // skip out of bounds
    //             if nx < 0 || nx >= self.width as i32 { continue; }
    //             // skip out of bounds
    //             if ny < 0 || ny >= self.height as i32 { continue; }
    //
    //             count += if self.data[ny as usize][nx as usize] == item { 1 } else { 0 };
    //         }
    //     }
    //     count
    // }

    pub fn print(&self) {
        let result = self.data.iter()
            .map(|row| row.iter().cloned().map(|n| n.to_string()).collect::<String>() + "\n")
            .collect::<String>();
        println!("{}", result);
    }
}
