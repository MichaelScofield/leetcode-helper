mod helper;

struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        assert!(s.len() > 0);
        let letters: Vec<u8> = s.as_bytes().iter().map(|b| *b - 97).collect();
        let mut counts = vec![0; 26];
        for letter in letters {
            counts[letter as usize] += 1;
        }
        counts.sort();

        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut deletions = 0;
        for i in 0..counts.len() {
            let c = counts[i];
            if c == 0 {
                continue;
            }
            if set.insert(c) {
                continue;
            }
            for j in (0..c).rev() {
                if j == 0 || set.insert(j) {
                    deletions += c - j;
                    break;
                }
            }
        }
        deletions as i32
    }
}

fn main() {
    assert_eq!(0, Solution::min_deletions("aab".to_string()));
    assert_eq!(2, Solution::min_deletions("aaabbbcc".to_string()));
    assert_eq!(2, Solution::min_deletions("ceabaacb".to_string()));
}
