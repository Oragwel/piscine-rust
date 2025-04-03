pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        // Extract the number from each word
        word.chars()
            .filter_map(|c| c.to_digit(10)) // Extract digits from the word
            .next()                         // Get the first digit (position of the word)
            .unwrap_or(0)                   // Default to 0 if no number is found (shouldn't happen)
    });

    words.join(" ")
}
