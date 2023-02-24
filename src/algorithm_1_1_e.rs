//! Algorithm 1.1E: The Euclid's algorithm.
//!
//! Given two positive integers `m` and `n`, find their _greatest common
//! divisor_, that is, the largest positive integer that evenly divides
//! both `m` and `n`.

pub fn greatest_common_divisor_loop(mut a: u32, mut b: u32) -> u32 {
    loop {
        let r = a % b;
        if r == 0 {
            return b;
        }
        a = b;
        b = r;
    }
}

pub fn greatest_common_divisor_recursion(a: u32, b: u32) -> u32 {
    let r = a % b;
    match r {
        0 => b,
        _ => greatest_common_divisor_recursion(b, r),
    }
}

#[cfg(test)]
mod test {
    use super::greatest_common_divisor_loop;
    use super::greatest_common_divisor_recursion;

    #[test]
    fn test_loop() {
        assert_eq!(greatest_common_divisor_loop(1, 1), 1);
        assert_eq!(greatest_common_divisor_loop(12, 8), 4);
        assert_eq!(greatest_common_divisor_loop(8, 12), 4);
    }

    #[test]
    fn test_recursion() {
        assert_eq!(greatest_common_divisor_recursion(1, 1), 1);
        assert_eq!(greatest_common_divisor_recursion(12, 8), 4);
        assert_eq!(greatest_common_divisor_recursion(8, 12), 4);
    }
}
