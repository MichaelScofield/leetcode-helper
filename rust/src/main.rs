mod helper;

struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        assert!(n >= 1 && s.len() > n as usize);
        let mut chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();
        let n = n as usize;
        chars[0..len].reverse();
        chars[0..len - n].reverse();
        chars[len - n..len].reverse();
        chars.iter().collect()
    }
}

fn main() {
    assert_eq!("cdefgab".to_string(), Solution::reverse_left_words("abcdefg".to_string(), 2));
    assert_eq!("umghlrlose".to_string(), Solution::reverse_left_words("lrloseumgh".to_string(), 6));
}
