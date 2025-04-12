pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let offset = (key.rem_euclid(26)) as u8;
                ((((c as u8 - b'a') + offset) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                let offset = (key.rem_euclid(26)) as u8;
                ((((c as u8 - b'A') + offset) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect()
}
