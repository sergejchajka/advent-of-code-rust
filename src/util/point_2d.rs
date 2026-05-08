use crate::util::math::round;

pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    // pub fn new(x: i64, y: i64) -> Self {
    //     Self { x, y }
    // }

    pub fn euclidean_distance(&self, other: Point2D) -> f64 {
        let distance = ((
            (other.x - self.x).pow(2) +
            (other.y - self.y).pow(2)
        ) as f64).sqrt();

        round(distance, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let point1 = Point2D { x: 4, y: 1 };
        let point2 = Point2D { x: 3, y: 0 };

        assert_eq!(point1.euclidean_distance(point2), 1.41421_f64)
    }
}
