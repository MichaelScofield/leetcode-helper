mod helper;

struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        assert!(s.len() >= 1);

        let s = s.as_bytes();
        let n = s.len();
        let mut palindromes = vec![vec![false; n]; n];
        for i in 0..n {
            palindromes[i][i] = true;
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                if j - i == 1 {
                    palindromes[i][j] = s[i] == s[j];
                } else {
                    palindromes[i][j] = palindromes[i + 1][j - 1] && (s[i] == s[j]);
                }
            }
        }

        let mut dp = vec![n; n];
        dp[0] = 0;
        for i in 1..n {
            for p in 0..=i {
                if palindromes[p][i] {
                    if p == 0 {
                        dp[i] = 0;
                        break;
                    } else {
                        dp[i] = std::cmp::min(dp[p - 1] + 1, dp[i]);
                    }
                }
            }
        }
        dp[n - 1] as i32
    }
}

fn main() {
    assert_eq!(1, Solution::min_cut("aaba".to_string()));
    assert_eq!(0, Solution::min_cut("aabaa".to_string()));
    assert_eq!(1, Solution::min_cut("aab".to_string()));
    assert_eq!(0, Solution::min_cut("a".to_string()));
    assert_eq!(1, Solution::min_cut("ab".to_string()));
}
