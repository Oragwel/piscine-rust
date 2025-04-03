pub fn doubtful(s: &mut String) {
    s.push('?');
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubtful() {
        let mut s = String::from("Hello");
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
        
        let mut s2 = String::from("Rust");
        doubtful(&mut s2);
        assert_eq!(s2, "Rust?");
        
        let mut s3 = String::from("");
        doubtful(&mut s3);
        assert_eq!(s3, "?"); // Test empty string case
    }
}

