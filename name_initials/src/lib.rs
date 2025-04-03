pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace() // Split into words
                .map(|part| format!("{}.", part.chars().next().unwrap())) // Append `.` directly
                .collect::<Vec<_>>() // Collect into a Vec
                .join(" ") // Join efficiently
        })
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initials() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let expected = vec!["H. P.", "S. E.", "J. L.", "B. O."];

        assert_eq!(initials(names), expected);
    }
}

