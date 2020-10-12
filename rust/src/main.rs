mod helper;

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut dp = vec![vec![0; len]; len];
        for j in 0..len {
            for i in (0..j).rev() {
                if bytes[i] == bytes[j] {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    dp[i][j] = std::cmp::min(dp[i + 1][j], dp[i][j - 1]) + 1;
                }
            }
        }
        dp[0][len - 1]
    }
}

fn main() {
    assert_eq!(0, Solution::min_insertions("zzazz".to_string()));
    assert_eq!(2, Solution::min_insertions("mbadm".to_string()));
    assert_eq!(5, Solution::min_insertions("leetcode".to_string()));
    assert_eq!(0, Solution::min_insertions("g".to_string()));
    assert_eq!(1, Solution::min_insertions("no".to_string()));
}
