pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    let n = (c as u8 - b'A' + 1) as usize;
    for i in 0..n {
        let spaces = " ".repeat(n - i - 1);
        let mut s = format!("{}{}", spaces, (b'A' + i as u8) as char);
        if i > 0 {
            let inner_spaces = " ".repeat(2 * i - 1);
            s = format!("{}{}{}", s, inner_spaces, (b'A' + i as u8) as char);
        }
        s = format!("{}{}", s, spaces);
        result.push(s);
    }
    for i in (0..n - 1).rev() {
        let spaces = " ".repeat(n - i - 1);
        let mut s = format!("{}{}", spaces, (b'A' + i as u8) as char);
        if i > 0 {
            let inner_spaces = " ".repeat(2 * i - 1);
            s = format!("{}{}{}", s, inner_spaces, (b'A' + i as u8) as char);
        }
        s = format!("{}{}", s, spaces);
        result.push(s);
    }
    result
}