pub fn delete_and_backspace(s: &mut String) {
    let mut final_string = String::new();
    let mut counter = 0;

    for character in s.chars() {
        if counter != 0 && character != '+' {
            counter -= 1;
            continue;
        }
        match character {
            '-' => {
                final_string.pop(); // backspace
            },
            '+' => {
                counter += 1; // delete
            },
            _ => {
                final_string.push(character); // normal character
            },
        }
    }

    s.clear();
    s.push_str(&final_string);
}

pub fn do_operations(v: &mut Vec<String>) {
    for i in 0..v.len() {
        let equation = &v[i];

        if equation.contains('+') {
            let operands: Vec<&str> = equation.split('+').collect();
            if operands.len() == 2 {
                let left = operands[0].trim().parse::<i32>().unwrap_or(0);
                let right = operands[1].trim().parse::<i32>().unwrap_or(0);
                let result = left + right;
                v[i] = result.to_string();
            }
        } else if equation.contains('-') {
            let operands: Vec<&str> = equation.split('-').collect();
            if operands.len() == 2 {
                let left = operands[0].trim().parse::<i32>().unwrap_or(0);
                let right = operands[1].trim().parse::<i32>().unwrap_or(0);
                let result = left - right;
                v[i] = result.to_string();
            }
        }
    }
}
