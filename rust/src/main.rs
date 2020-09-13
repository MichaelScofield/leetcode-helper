mod helper;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        if s.len() == 0 {
            return -1;
        }
        let chars = s.as_bytes();
        let mut index = vec![(0, -1); 26];
        for i in 0..chars.len() {
            let offset = (chars[i] - 97) as usize;
            index[offset].0 += 1;
            index[offset].1 = i as i32;
        }
        index.iter().filter(|(count, _)| *count == 1).map(|(_, pos)| *pos).min().unwrap_or(-1)
    }
}

fn main() {
    assert_eq!(-1, Solution::first_uniq_char("cc".to_string()));
    assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
    assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
}
