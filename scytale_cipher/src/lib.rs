pub fn scytale_cipher(message: String, i: u32) -> String {
    let padded_message = format!(
        "{:<width$}",
        message,
        width = (message.len() as f32 / i as f32).ceil() as usize * i as usize
    );
    (0..i)
        .flat_map(|j| padded_message.chars().skip(j as usize).step_by(i as usize))
        .collect::<String>()
        .trim()
        .to_string()
}