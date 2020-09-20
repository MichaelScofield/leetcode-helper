mod helper;

struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let len = s.len();
        assert!(len > 0);
        let s = s.as_str();
        use std::collections::HashSet;
        let mut substrs = HashSet::new();
        let mut i = 0;
        let mut j = i + 1;
        while j <= len {
            if !substrs.contains(&s[i..j]) {
                substrs.insert(&s[i..j]);
                i = j;
            }
            j += 1;
        }
        substrs.len() as i32
    }
}

fn main() {
    assert_eq!(5, Solution::max_unique_split("addbsd".to_string()));
    assert_eq!(5, Solution::max_unique_split("ababccc".to_string()));
    assert_eq!(2, Solution::max_unique_split("aba".to_string()));
    assert_eq!(1, Solution::max_unique_split("aa".to_string()));
}
