mod helper;

struct Solution;

impl Solution {
    /*
    s1 --(num)--> s3 (Terminal)
    s1 --(+/-)--> s2
    s1 --(.)--> s7

    s2 --(num)--> s3 (Terminal)
    s2 --(.)--> s7

    s3 --(num)--> s3 (Terminal)
    s3 --(e/E)--> s4
    s3 --(.)--> s7 (Terminal)

    s4 --(empty/+/-)--> s5

    s5 --(num)--> s6 (Terminal)

    s6 --(num)--> s6 (Terminal)

    s7 --(num)--> s8

    s8 --(num)--> s8 (Terminal)
    s8 --(e/E)--> s4
    */
    pub fn is_number(s: String) -> bool {
        if s.len() == 0 {
            return false;
        }
        let is_sign = |c| { c == '+' || c == '-' };
        let is_dot = |c| { c == '.' };
        let is_e = |c| { c == 'e' || c == 'E' };
        let s: Vec<char> = s.trim().chars().collect();
        let n = s.len();
        if n == 1 && s[0] == '.' {
            return false;
        }
        let mut state = 1;
        let mut i = 0;
        while i < n {
            let c = s[i];
            i += 1;
            match state {
                1 => {
                    if c.is_ascii_digit() {
                        state = 3;
                    } else if is_sign(c) {
                        state = 2;
                    } else if is_dot(c) {
                        state = 7;
                    } else {
                        return false;
                    }
                }
                2 => {
                    if c.is_ascii_digit() {
                        state = 3;
                    } else if is_dot(c) {
                        state = 7;
                    } else {
                        return false;
                    }
                }
                3 => {
                    if c.is_ascii_digit() {
                        state = 3;
                    } else if is_e(c) {
                        state = 4;
                    } else if is_dot(c) {
                        state = 7;
                    } else {
                        return false;
                    }
                }
                4 => {
                    if c.is_ascii_digit() {
                        i -= 1;
                        state = 5;
                    } else if is_sign(c) {
                        state = 5;
                    } else {
                        return false;
                    }
                }
                5 | 6 => {
                    if c.is_ascii_digit() {
                        state = 6;
                    } else {
                        return false;
                    }
                }
                7 => {
                    if c.is_ascii_digit() {
                        state = 8;
                    } else {
                        return false;
                    }
                }
                8 => {
                    if c.is_ascii_digit() {
                        state = 8;
                    } else if is_e(c) {
                        state = 4;
                    } else {
                        return false;
                    }
                }
                _ => panic!("Illegal state!")
            }
        }
        state == 3 || state == 6 || state == 7 || state == 8
    }
}

fn main() {
    assert!(Solution::is_number("+100".to_string()));
    assert!(Solution::is_number("5e2".to_string()));
    assert!(Solution::is_number("-123".to_string()));
    assert!(Solution::is_number("3.1416".to_string()));
    assert!(Solution::is_number("-1E-16".to_string()));
    assert!(Solution::is_number("0123".to_string()));
    assert!(!Solution::is_number("12e".to_string()));
    assert!(!Solution::is_number("1a3.14".to_string()));
    assert!(!Solution::is_number("1.2.3".to_string()));
    assert!(!Solution::is_number("+-5".to_string()));
    assert!(!Solution::is_number("12e+5.4".to_string()));
    assert!(Solution::is_number("1".to_string()));
    assert!(Solution::is_number("1 ".to_string()));
    assert!(Solution::is_number(" 1 ".to_string()));
    assert!(Solution::is_number(" .1 ".to_string()));
    assert!(!Solution::is_number(" .1. ".to_string()));
    assert!(Solution::is_number(" 1. ".to_string()));
    assert!(!Solution::is_number(" . ".to_string()));
}
