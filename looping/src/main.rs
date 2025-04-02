use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. \
                  I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut attempts = 0;

    loop {
        println!("{}", riddle);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_input = user_input.trim();

        attempts += 1;

        if user_input.eq_ignore_ascii_case(answer) {
            println!("Number of trials: {}", attempts);
            break;
        }
    }
}
