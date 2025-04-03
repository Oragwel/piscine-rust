pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace() // Split the name into words
                .filter_map(|part| part.chars().next()) // Get the first character of each part
                .map(|c| format!("{}.", c)) // Append a dot to each initial
                .collect::<Vec<String>>() // Collect into a vector of initials
                .join(" ") // Join with a space between initials
        })
        .collect() // Collect into a Vec<String>
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

