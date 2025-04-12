pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    let mut chars: Vec<char> = message.chars().collect();
    
    // Pad the message with spaces to fill the last row if necessary
    let remainder = chars.len() % i;
    if remainder != 0 {
        chars.extend(vec![' '; i - remainder]);
    }

    let rows = chars.len() / i;
    let mut result = String::with_capacity(chars.len());

    for col in 0..i {
        for row in 0..rows {
            result.push(chars[row * i + col]);
        }
    }

    result
}
