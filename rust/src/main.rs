mod helper;

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        let mut add = 0;
        for &x in s.as_bytes() {
            if x == b'(' {
                stack.push(x);
            } else {
                if let Some(&p) = stack.last() {
                    if p == b'(' {
                        stack.pop();
                    } else {
                        add += 1;
                    }
                } else {
                    add += 1;
                }
            }
        }
        add + stack.len() as i32
    }
}

fn main() {
    assert_eq!(1, Solution::min_add_to_make_valid("())".to_string()));
    assert_eq!(3, Solution::min_add_to_make_valid("(((".to_string()));
    assert_eq!(0, Solution::min_add_to_make_valid("()".to_string()));
    assert_eq!(4, Solution::min_add_to_make_valid("()))((".to_string()));
}
