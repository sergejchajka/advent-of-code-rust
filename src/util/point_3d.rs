use std::fmt;
use crate::util::math::round;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    // pub fn new(x: i64, y: i64, z: i64) -> Self {
    //     Self { x, y, z }
    // }

    pub fn euclidean_distance(&self, other: &Point3D) -> f64 {
        let distance = ((
            (self.x - other.x).pow(2) +
            (self.y - other.y).pow(2) +
            (self.z - other.z).pow(2)
        ) as f64).sqrt();

        round(distance, 5)
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let point1 = Point3D { x: 0, y: 0, z: 0 };
        let point2 = Point3D { x: 1, y: 2, z: 3 };

        assert_eq!(point1.euclidean_distance(&point2), 3.74166)
    }
}
