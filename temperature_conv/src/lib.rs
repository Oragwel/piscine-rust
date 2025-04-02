pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fahrenheit_to_celsius_works() {
        let result = fahrenheit_to_celsius(-459.67);
        assert_eq!(result, -273.15);
    }

    #[test]
    fn celsius_to_fahrenheit_works() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }
}
