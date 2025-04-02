pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h / 3.6
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn test_km_per_hour_to_meters_per_second() {
        let speed_km_h = 100.0;
        let expected_m_s = 27.77777777777778;
        let result = km_per_hour_to_meters_per_second(speed_km_h);
        assert!((result - expected_m_s).abs() < EPSILON, "Expected {} but got {}", expected_m_s, result);
    }
}
