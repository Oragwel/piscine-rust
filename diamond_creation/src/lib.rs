pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let mut diamond = Vec::new();

    for i in 0..=n {
        let letter = (b'A' + i as u8) as char;
        let outer_spaces = n - i;
        let inner_spaces = if i == 0 { 0 } else { 2 * i - 1 };
        let line = if i == 0 {
            format!("{:width$}", letter, width = n + 1)
        } else {
            format!(
                "{:outer$}{}{:inner$}{}",
                "",
                letter,
                "",
                letter,
                outer = outer_spaces,
                inner = inner_spaces
            )
        };
        diamond.push(line);
    }

    for i in (0..n).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}
