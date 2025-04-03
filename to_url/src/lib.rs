pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        assert_eq!(to_url("Hello, world!"), "Hello,%20world!");
        assert_eq!(to_url("Rust is awesome"), "Rust%20is%20awesome");
        assert_eq!(to_url("NoSpacesHere"), "NoSpacesHere");  // No spaces to replace
        assert_eq!(to_url(" Leading and trailing spaces "), "%20Leading%20and%20trailing%20spaces%20");  // Test spaces at the ends
        assert_eq!(to_url("Multiple  spaces"), "Multiple%20%20spaces");  // Test multiple consecutive spaces
    }
}

