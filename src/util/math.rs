pub fn round(value: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

pub fn factorise(number: u32) -> Vec<u32> {
    if number <= 1 {
        return vec![number];
    }

    let mut factors = Vec::new();
    let mut value = number;

    while value % 2 == 0 {
        factors.push(2);
        value /= 2;
    }

    for delimiter in (3..value).step_by(2) {
        while value % delimiter == 0 {
            factors.push(delimiter);
            value /= delimiter;
        }
    }

    if value > 1 || number == 1 {
        factors.push(value);
    }

    factors
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_factorise() {
        assert_eq!(factorise(10), vec![2, 5]);
        assert_eq!(factorise(15), vec![3, 5]);
        assert_eq!(factorise(21), vec![3, 7]);
        assert_eq!(factorise(4), vec![2, 2]);
        assert_eq!(factorise(3), vec![3]);
        assert_eq!(factorise(2), vec![2]);
        assert_eq!(factorise(1), vec![1]);
    }

    #[test]
    fn test_factorise_beltmatick() {
        assert_eq!(factorise(305), vec![1]); //  [5, 61]
    }

    #[test]
    fn closest_exponent() {
        let target_value = 11616;

        let numbers: Vec<u32> = vec![2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14];
        let numbers2: Vec<u32> = vec![2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14];

        let mut closest_lower: (u32, u32, u32) = (u32::MIN, 0, 0);
        let mut closest_bigger: (u32, u32, u32) = (u32::MAX, 0, 0);

        'outer: for number1 in numbers {
            'inner: for number2 in numbers2.clone() {
                let value = number1.pow(number2);
                // dbg!(format!("{:?}.pow({:?}) = {:?}", number1, number2, value));
                // sleep(Duration::from_millis(500));
                if value <= target_value && closest_lower.0 < value {
                    closest_lower = (value, number1, number2);
                }

                if value >= target_value && closest_bigger.0 > value {
                    closest_bigger = (value, number1, number2);
                }

                if value == target_value {
                    break 'outer;
                }

                if value > target_value {
                    break 'inner;
                }
            }
        }


        println!("Target value: {:?}, factors: {:?}", target_value, factorise(target_value));
        println!("Closest lower: {:?}, plus: {:?} {:?}",
                 closest_lower,
                 target_value - closest_lower.0,
                 factorise(target_value - closest_lower.0));
        println!("Closest bigger: {:?}, minus: {:?} {:?}",
                 closest_bigger,
                 closest_bigger.0 - target_value,
                 factorise(closest_bigger.0 - target_value));
    }

}
