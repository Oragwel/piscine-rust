pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        word.chars()
            .filter_map(|c| c.to_digit(10)) // Extract digits from each word
            .next()                         // The first digit is the position of the word
            .unwrap_or(0)                   // If no digit is found, default to 0 (though this shouldn't happen)
    });

    words.join(" ")
}
