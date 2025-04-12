pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";

    // Handle the case where the word starts with a vowel
    if vowels.contains(text.chars().next().unwrap()) {
        return format!("{}ay", text);
    }

    // Handle words starting with consonants
    let mut chars = text.chars();
    let mut prefix = String::new();

    // Handle "qu" case
    if chars.clone().take(2).collect::<String>() == "qu" {
        prefix = chars.take(2).collect();
        return format!("{}ay", text[2..].to_string() + &prefix);
    }

    // Handle consonant to first vowel case
    for (i, ch) in text.chars().enumerate() {
        if vowels.contains(ch) {
            let rest_of_word = &text[i..];
            return format!("{}{}ay", &text[0..i], rest_of_word);
        }
    }

    text.to_string()  // Fallback if no vowel is found (unlikely for normal words)
}

