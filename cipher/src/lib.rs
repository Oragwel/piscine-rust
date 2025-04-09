#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

fn atbash_char(c: char) -> char {
    if c.is_ascii_lowercase() {
        // 'a' to 'z' → 'z' to 'a'
        (b'z' - (c as u8 - b'a')) as char
    } else if c.is_ascii_uppercase() {
        // 'A' to 'Z' → 'Z' to 'A'
        (b'Z' - (c as u8 - b'A')) as char
    } else {
        c
    }
}

fn atbash_cipher(s: &str) -> String {
    s.chars().map(atbash_char).collect()
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = atbash_cipher(original);
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
