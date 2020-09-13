mod helper;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut longest = 1;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut dp = vec![0; n];
        dp[0] = 1;
        for i in 1..n {
            let mut dup = None;
            for j in i - dp[i - 1]..i {
                if chars[j] == chars[i] {
                    dup = Some(j);
                    break;
                }
            }
            if let Some(index) = dup {
                dp[i] = i - index;
            } else {
                dp[i] = dp[i - 1] + 1;
            }
            longest = std::cmp::max(longest, dp[i]);
        }
        longest as i32
    }
}

fn main() {
    assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
    assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
    assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
    assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
}
