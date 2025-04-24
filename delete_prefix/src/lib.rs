pub fn delete_prefix(prefix: &str, s: &str) -> Option<&str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])  // Return the string slice without the prefix
    } else {
        None  // Return None if the string does not start with the prefix
    }
}
