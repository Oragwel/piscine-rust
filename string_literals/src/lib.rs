pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(usize::MAX)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_empty("Hello"), false);
    }

    #[test]
    fn test_is_ascii() {
        assert_eq!(is_ascii("Hello"), true);
        assert_eq!(is_ascii("Olá"), false);
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains("Hello, world!", "world"), true);
        assert_eq!(contains("Hello, world!", "Rust"), false);
    }

    #[test]
    fn test_split_at() {
        let (first, second) = split_at("Hello, world!", 5);
        assert_eq!(first, "Hello");
        assert_eq!(second, ", world!");
    }

    #[test]
    fn test_find() {
        assert_eq!(find("Hello, world!", 'o'), 4);
        assert_eq!(find("Hello, world!", 'z'), usize::MAX);  // Not found
    }
}
