pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; // No change
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j - 1], // Substitute
                    std::cmp::min(
                        dp[i - 1][j], // Delete
                        dp[i][j - 1],
                    ), // Insert
                );
            }
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = edit_distance("alignment", "assignment");
        assert_eq!(result, 2);
    }
}
