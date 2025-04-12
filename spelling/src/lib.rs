pub fn spell(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "ten", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];
    let thousands = ["", "thousand", "million"];
    let hundreds = ["", "hundred"];

    let mut result_parts = Vec::new();
    let mut i = 0;

    while n > 0 {
        let mut chunk = (n % 1000) as usize;
        n /= 1000;

        if chunk > 0 {
            let mut chunk_str = String::new();

            if chunk >= 100 {
                chunk_str.push_str(units[chunk / 100]);
                chunk_str.push_str(" ");
                chunk_str.push_str(hundreds[1]);
                chunk %= 100;
                if chunk > 0 {
                    chunk_str.push_str(" ");
                }
            }

            if chunk >= 11 && chunk <= 19 {
                chunk_str.push_str(teens[chunk - 10]);
            } else {
                if chunk >= 20 {
                    chunk_str.push_str(tens[chunk / 10]);
                    chunk %= 10;
                    if chunk > 0 {
                        chunk_str.push_str("-");
                    }
                }
                if chunk > 0 {
                    chunk_str.push_str(units[chunk]);
                }
            }

            if !thousands[i].is_empty() {
                if !chunk_str.is_empty() {
                    chunk_str.push_str(" ");
                }
                chunk_str.push_str(thousands[i]);
            }

            result_parts.push(chunk_str);
        }

        i += 1;
    }

    result_parts.reverse();
    result_parts.join(" ").trim().to_string()
}
