mod helper;

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        fn longest_common_subsequence(text1: &[u8], text2: &[u8]) -> usize {
            let m = text1.len();
            let n = text2.len();
            let mut dp = vec![vec![0; n + 1]; m + 1];
            for i in 1..=m {
                for j in 1..=n {
                    dp[i][j] = std::cmp::max(
                        dp[i - 1][j - 1] + if text1[i - 1] == text2[j - 1] { 1 } else { 0 },
                        std::cmp::max(dp[i][j - 1], dp[i - 1][j]));
                }
            }
            dp[m][n]
        }
        let text1 = word1.as_bytes();
        let text2 = word2.as_bytes();
        let lcs = longest_common_subsequence(text1, text2);
        (text1.len() - lcs + text2.len() - lcs) as i32
    }
}

fn main() {
    assert_eq!(2, Solution::min_distance("sea".to_string(), "eat".to_string()));
    assert_eq!(3, Solution::min_distance("sea".to_string(), "".to_string()));
    assert_eq!(0, Solution::min_distance("sea".to_string(), "sea".to_string()));
    assert_eq!(4, Solution::min_distance("sea".to_string(), "b".to_string()));
}
