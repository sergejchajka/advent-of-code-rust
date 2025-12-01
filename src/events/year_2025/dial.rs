use std::ops::{Add, Sub};

pub enum RotationDirection {
    Left,
    Right
}

impl From<&str> for RotationDirection {
    fn from(c: &str) -> Self {
        match c {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Invalid rotation direction: {}", c)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dial {
    pub position: i32,
    pub zero_crossed: u32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { position: 50, zero_crossed: 0 }
    }
}

impl Dial {
    pub fn new(position: i32) -> Self {
        Self {
            position,
            zero_crossed: 0,
        }
    }

    pub fn rotate(&mut self, direction: RotationDirection, steps: u32) {
        let mut new_position = self.position;

        // simplify to avoid unnecessary loops in for loop below
        let mut zero_crossed = steps / 100;
        let steps = steps % 100;

        for _ in 0..steps {
            match direction {
                RotationDirection::Left => new_position -= 1,
                RotationDirection::Right => new_position += 1,
            }
            new_position %= 100;
            if new_position == 0 {
                zero_crossed += 1
            }
        }

        if new_position < 0 {
            new_position += 100;
        }

        self.position = new_position;
        self.zero_crossed += zero_crossed;
    }
}

// sample implementation
impl Add for Dial {
    type Output = Self;

    fn add(self, other: Dial) -> Self::Output {
        let mut new_position = self.position + other.position;
        while new_position >= 100 {
            new_position -= 100 }
        Self {
            position: new_position,
            zero_crossed: 0,
        }
    }
}

// sample implementation
impl Sub for Dial {
    type Output = Self;

    fn sub(self, other: Dial) -> Self::Output {
        let mut new_position = self.position - other.position;
        while new_position < 0 { new_position += 100 }
        Self {
            position: new_position,
            zero_crossed: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::year_2025::dial::Dial;
    use crate::events::year_2025::RotationDirection::{Left, Right};

    #[test]
    fn test_add() {
        assert_eq!(Dial::new(0) + Dial::new(0), Dial::new(0));
        assert_eq!(Dial::new(0) + Dial::new(5), Dial::new(5));
        assert_eq!(Dial::new(0) + Dial::new(100), Dial::new(0));
        assert_eq!(Dial::new(10) + Dial::new(10), Dial::new(20));
        assert_eq!(Dial::new(99) + Dial::new(1), Dial::new(0));
        assert_eq!(Dial::new(0) + Dial::new(501), Dial::new(1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Dial::new(0) - Dial::new(1), Dial::new(99));
        assert_eq!(Dial::new(0) - Dial::new(100), Dial::new(0));
        assert_eq!(Dial::new(10) - Dial::new(10), Dial::new(0));
        assert_eq!(Dial::new(20) - Dial::new(10), Dial::new(10));
        assert_eq!(Dial::new(0) - Dial::new(300), Dial::new(0));
        assert_eq!(Dial::new(0) - Dial::new(310), Dial::new(90));
    }

    #[test]
    fn test_default() {
        let dial = Dial::default();
        assert_eq!(50, dial.position);
        assert_eq!(0, dial.zero_crossed);
    }

    #[test]
    fn test_rotate_right() {
        let mut dial = Dial::default();

        dial.rotate(Right, 0);
        assert_eq!(dial, Dial { position: 50, zero_crossed: 0 });

        dial.rotate(Right, 5);
        assert_eq!(dial, Dial { position: 55, zero_crossed: 0 });

        dial.rotate(Right, 45);
        assert_eq!(dial, Dial { position: 0, zero_crossed: 1 });

        dial.rotate(Right, 0);
        assert_eq!(dial, Dial { position: 0, zero_crossed: 1 });

        dial.rotate(Right, 100);
        assert_eq!(dial, Dial { position: 0, zero_crossed: 2 });

        dial.rotate(Right, 505);
        assert_eq!(dial, Dial { position: 5, zero_crossed: 7 });
    }

    #[test]
    fn test_rotate_left() {
        let mut dial = Dial::default();

        dial.rotate(Left, 0);
        assert_eq!(dial, Dial { position: 50, zero_crossed: 0 });

        dial.rotate(Left, 5);
        assert_eq!(dial, Dial { position: 45, zero_crossed: 0 });

        dial.rotate(Left, 45);
        assert_eq!(dial, Dial { position: 0, zero_crossed: 1 });

        dial.rotate(Left, 0);
        assert_eq!(dial, Dial { position: 0, zero_crossed: 1 });

        dial.rotate(Left, 100);
        assert_eq!(dial, Dial { position: 0, zero_crossed: 2 });

        dial.rotate(Left, 495);
        assert_eq!(dial, Dial { position: 5, zero_crossed: 6 });

        dial.rotate(Left, 10);
        assert_eq!(dial, Dial { position: 95, zero_crossed: 7 });
    }
}
