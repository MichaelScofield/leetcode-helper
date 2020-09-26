mod helper;

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut left = 0;
        let mut insertions = 0;
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            if bytes[i] == b'(' {
                // reserve '('
                left += 1;
            } else {
                if left > 0 {
                    // consume '('
                    left -= 1;
                } else {
                    // insert '('
                    insertions += 1;
                }
                if i + 1 < len {
                    if bytes[i + 1] == b')' {
                        // skip continuous ')'
                        i += 1;
                    } else {
                        // insert ')'
                        insertions += 1;
                    }
                } else {
                    // insert ')'
                    insertions += 1;
                }
            }
            i += 1;
        }
        insertions + left * 2
    }
}

fn main() {
    assert_eq!(1, Solution::min_insertions("(()))".to_string()));
    assert_eq!(0, Solution::min_insertions("())".to_string()));
    assert_eq!(3, Solution::min_insertions("))())(".to_string()));
    assert_eq!(12, Solution::min_insertions("((((((".to_string()));
    assert_eq!(5, Solution::min_insertions(")))))))".to_string()));
    assert_eq!(3, Solution::min_insertions("()()))".to_string()));
    assert_eq!(0, Solution::min_insertions("(())))".to_string()));
    assert_eq!(4, Solution::min_insertions(")()))".to_string()));
    assert_eq!(4, Solution::min_insertions("(()))(()))()())))".to_string()));
}
