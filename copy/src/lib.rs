use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_val = f64::exp(c as f64);
    let log_val = if c == 0 { f64::NEG_INFINITY } else { (c as f64).abs().ln() };
    (c, exp_val, log_val)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_vals: Vec<String> = a
        .split_whitespace()
        .map(|num_str| {
            let num: f64 = num_str.parse().unwrap();
            f64::exp(num).to_string()
        })
        .collect();
    (a, exp_vals.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log_vals: Vec<f64> = b.iter().map(|&x| (x as f64).abs().ln()).collect();
    (b, log_vals)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbr_function() {
        let result = nbr_function(0);
        assert_eq!(result, (0, 1.0, f64::NEG_INFINITY));
    }

    #[test]
    fn test_vec_function() {
        let result = vec_function(vec![1, 2, 4, 5]);
        assert_eq!(result, (vec![1, 2, 4, 5], vec![0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }

    #[test]
    fn test_str_function() {
        let result = str_function("1 2 4 5 6".to_owned());
        assert_eq!(result, ("1 2 4 5 6".to_owned(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_owned()));
    }
}
