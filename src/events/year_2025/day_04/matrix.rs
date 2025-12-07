#[derive(Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<char>>,
}
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Matrix {
    
    pub fn from_input(input: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        input.lines().for_each( |line| data.push(line.chars().collect()));
        let width = data[0].len();
        let height = data.len();

        Self {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    pub(crate) fn get_line(&self, y: usize) -> Vec<char> {
        self.data[y].clone()
    }

    pub fn replace(&mut self, x: usize, y: usize, item: char) {
        self.data[y][x] = item
    }
    
    pub fn count_adjacent_items_count(&self, x: usize, y: usize, item: char) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                // skip itself
                if dx == 0 && dy == 0 { continue; }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // skip out of bounds
                if nx < 0 || nx >= self.width as i32 { continue; }
                // skip out of bounds
                if ny < 0 || ny >= self.height as i32 { continue; }

                count += if self.data[ny as usize][nx as usize] == item { 1 } else { 0 };
            }
        }
        count
    }

    pub fn print(&self) {
        let result = self.data.iter()
            .map(|row| row.iter().cloned().collect::<String>() + "\n")
            .collect::<String>();
        print!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_adjacent_items_count() {
        let matrix = Matrix::from_input(
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.");

        assert_eq!(matrix.count_adjacent_items_count(2, 0, '@'), 3);
        assert_eq!(matrix.count_adjacent_items_count(6, 2, '@'), 2);
    }
}
