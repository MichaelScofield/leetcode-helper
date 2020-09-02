mod helper;

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        if m == 0 {
            return n as i32;
        }
        if n == 0 {
            return m as i32;
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i;
        }
        for i in 0..=n {
            dp[0][i] = i;
        }
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        for i in 1..=m {
            for j in 1..=n {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    let del = dp[i - 1][j] + 1;
                    let insert = dp[i][j - 1] + 1;
                    let replace = dp[i - 1][j - 1] + 1;
                    dp[i][j] = std::cmp::min(std::cmp::min(del, insert), replace);
                }
            }
        }
        dp[m][n] as i32
    }
}

fn main() {
    assert_eq!(3, Solution::min_distance("horse".to_string(), "ros".to_string()));
    assert_eq!(5, Solution::min_distance("intention".to_string(), "execution".to_string()));
}
