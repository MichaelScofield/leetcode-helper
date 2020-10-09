mod helper;

struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut dp = vec![vec![0; len]; len];
        for i in 0..len {
            dp[i][i] = 1;
        }
        for j in 1..len {
            for i in (0..j).rev() {
                let mut equal_index = None;
                for x in i..j {
                    if bytes[x] == bytes[j] {
                        equal_index = Some(x);
                        break;
                    }
                }
                dp[i][j] = if let Some(x) = equal_index {
                    std::cmp::max(dp[x + 1][j - 1] + 2, dp[i][j - 1])
                } else {
                    dp[i][j - 1]
                }
            }
        }
        dp[0][len - 1]
    }
}

fn main() {
    assert_eq!(6, Solution::longest_palindrome_subseq("abaabaa".to_string()));
    assert_eq!(4, Solution::longest_palindrome_subseq("bbbab".to_string()));
    assert_eq!(2, Solution::longest_palindrome_subseq("cbbd".to_string()));
}
