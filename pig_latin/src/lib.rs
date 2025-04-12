pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";

    // Vowel case
    if let Some(first) = text.chars().next() {
        if vowels.contains(first) {
            return format!("{}ay", text);
        }
    }

    // Handle consonant + "qu"
    if let Some(pos) = text.find("qu") {
        if pos == 0 || !vowels.contains(text.chars().nth(pos - 1).unwrap_or(' ')) {
            return format!("{}quay", &text[2..]);
        }
    }

    // General consonant case
    let mut consonant_cluster = String::new();
    for (i, c) in text.chars().enumerate() {
        if vowels.contains(c) {
            return format!("{}{}ay", &text[i..], consonant_cluster);
        } else {
            consonant_cluster.push(c);
        }
    }

    // In case there are no vowels at all
    format!("{}ay", text)
}
