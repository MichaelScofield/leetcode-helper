mod helper;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let chars = s.chars().collect::<Vec<char>>();

        use std::collections::HashSet;
        let mut seen = HashSet::new();
        seen.insert(chars[0]);

        let mut longest = 1;

        let mut i = 0;
        let mut j = 1;
        while j < chars.len() {
            while j < chars.len() && seen.insert(chars[j]) {
                j += 1;
            }
            longest = std::cmp::max(longest, j - i);
            seen.remove(&chars[i]);
            i += 1;
        }
        longest as i32
    }
}

fn main() {
    assert_eq!(3, Solution::length_of_longest_substring("bwf".to_string()));
    assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    assert_eq!(
        1,
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
    assert_eq!(
        3,
        Solution::length_of_longest_substring("pwwkew".to_string())
    );
    assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
}
