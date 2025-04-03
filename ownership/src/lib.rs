pub fn first_subword(mut s: String) -> String {
    let mut end_index = s.len();

    for (i, c) in s.chars().enumerate() {
        if i > 0 && (c.is_uppercase() || c == '_') {
            end_index = i;
            break;
        }
    }

    s.truncate(end_index);
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_subword() {
        assert_eq!(first_subword("helloWorld".to_owned()), "hello");
        assert_eq!(first_subword("snake_case".to_owned()), "snake");
        assert_eq!(first_subword("CamelCase".to_owned()), "Camel");
        assert_eq!(first_subword("just".to_owned()), "just");
        assert_eq!(first_subword("PascalStyle".to_owned()), "Pascal");
    }
}
