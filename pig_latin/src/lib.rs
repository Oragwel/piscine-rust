pub fn pig_latin(text: &str) -> String {
    println!("{}", text);
    fn is_vowel(c: char) -> bool {
        match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            }
    }
    let mut new_string = text.to_string();
    let mut next_char = false;
    loop {
        for (i, char) in text.chars().enumerate() {
            if next_char && char == 'q' {
                new_string.remove(0);
                new_string.push(char);
                continue
            }
            if next_char && char == 'u' {
                new_string.remove(0);
                new_string.push(char);
                next_char = false;
            }
            if !is_vowel(char) && text.len() > i + 2 && &text[i+1..i+3] == "qu" {
                new_string.remove(0);
                new_string.push(char);
                next_char = true;
            }else if i == 0 && is_vowel(char){
                break;
            }else if !is_vowel(char) {
                new_string.remove(0);
                new_string.push(char);
                continue;
            } else { 
                break
            }
        }
        break
    }
    new_string.push('a');
    new_string.push('y');
    return new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", pig_latin(&String::from("queen")));
    }
}