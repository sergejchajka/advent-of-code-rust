pub fn round(value: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

pub fn factorise(number: u32) -> Vec<u32> {
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
        assert_eq!(factorise(698), vec![2, 349]);
    }

}
