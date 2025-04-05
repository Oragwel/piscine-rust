pub fn sum(a: &[i32]) -> i32 {
    let mut sum = 0;
    for num in a.iter() {
        sum += num;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = (1..=10).collect::<Vec<i32>>();
        let b = [5;10];
        let result = sum(&a);
        assert_eq!(result, 55);
        let result1 = sum(&b);
        assert_eq!(result1, 50);
        let result2 = thirtytwo_tens();
        assert_eq!(
            result2,
            [
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10
            ]
        );
    }
}
