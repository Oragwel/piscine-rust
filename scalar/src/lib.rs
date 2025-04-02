pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b).expect("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b).expect("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("ERROR: division by zero");
    }
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("ERROR: division by zero");
    }
    a % b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(234, 2), 236);
    }

    #[test]
    #[should_panic(expected = "ERROR: attempt to add with overflow")]
    fn test_sum_overflow() {
        sum(1, 255);
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(234, 2), 232);
    }

    #[test]
    #[should_panic(expected = "ERROR: attempt to subtract with overflow")]
    fn test_diff_overflow() {
        diff(-32768, 32766);
    }

    #[test]
    fn test_pro() {
        assert_eq!(pro(23, 2), 46);
    }

    #[test]
    #[should_panic(expected = "ERROR: attempt to multiply with overflow")]
    fn test_pro_overflow() {
        pro(-128, 2);
    }

    #[test]
    fn test_quo() {
        assert_eq!(quo(22.0, 2.0), 11.0);
        assert_eq!(quo(-128.23, 2.0), -64.115);
    }

    #[test]
    #[should_panic(expected = "ERROR: division by zero")]
    fn test_quo_div_by_zero() {
        quo(10.0, 0.0);
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(22.0, 2.0), 0.0);
        assert_eq!(rem(-128.23, 2.0), -0.22999573);
    }

    #[test]
    #[should_panic(expected = "ERROR: division by zero")]
    fn test_rem_div_by_zero() {
        rem(10.0, 0.0);
    }
}

