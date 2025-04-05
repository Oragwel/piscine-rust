pub fn char_length(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = char_length("❤");
        assert_eq!(result, 1);
        let result1 = char_length("形聲字");
        assert_eq!(result1, 3);
        let result2 = char_length("change");
        assert_eq!(result2, 6);
        let result3 = char_length("😍");
        assert_eq!(result3, 1);
    }
}
