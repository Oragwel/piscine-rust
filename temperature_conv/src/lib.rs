pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EPSILON: f64 = 1e-6; // Set an epsilon value for floating-point comparison

    #[test]
    fn test_c_to_f() {
        let celsius = 0.0;
        let fahrenheit = celsius_to_fahrenheit(celsius);
        assert!((fahrenheit - 32.0).abs() < EPSILON, "Expected 32.0 but got {}", fahrenheit);
    }

    #[test]
    fn test_f_to_c() {
        let fahrenheit = 20.0;
        let celsius = fahrenheit_to_celsius(fahrenheit);
        assert!((celsius + 6.666666666666666).abs() < EPSILON, "Expected -6.666666666666666 but got {}", celsius);
    }
}
