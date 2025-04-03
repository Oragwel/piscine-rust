pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    
    // Use a loop to iterate over each character in the string
    for c in s.chars() {
        match c {
            '-' => {
                // Backspace: remove the last character from the result, if any
                if !result.is_empty() {
                    result.pop();
                }
            },
            '+' => {
                // Delete: ignore the character
                continue;
            },
            _ => {
                // Regular character: append to the result
                result.push(c);
            }
        }
    }

    // Replace the original string with the result
    *s = result;
}


pub fn do_operations(v: &mut [String]) {
    for equation in v.iter_mut() {
        let parts: Vec<&str> = equation.split(&['+', '-'][..]).collect();
        if parts.len() == 2 {
            let left = parts[0].parse::<i32>().unwrap_or(0);
            let right = parts[1].parse::<i32>().unwrap_or(0);
            let result = if equation.contains('+') {
                left + right
            } else {
                left - right
            };
            *equation = result.to_string();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
