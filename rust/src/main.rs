mod helper;

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let m = s1.len();
        let n = s2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] + s1[i - 1] as i32;
        }
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] + s2[j - 1] as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    dp[i][j] = std::cmp::min(
                        dp[i][j - 1] + s2[j - 1] as i32,
                        dp[i - 1][j] + s1[i - 1] as i32)
                }
            }
        }
        dp[m][n]
    }
}

fn main() {
    assert_eq!(231, Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()));
    assert_eq!(403, Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()));
}
