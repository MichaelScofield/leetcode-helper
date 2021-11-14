mod helper;

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut int = Vec::<char>::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let mut is_parsing_started = false;
        let mut is_first_nonzero_digit = false;
        let mut is_num_positive = true;
        for c in chars {
            match c {
                ' ' => {
                    if is_parsing_started {
                        break;
                    }
                }
                '-' | '+' => {
                    if is_parsing_started {
                        break;
                    }
                    is_num_positive = c == '+';
                    is_parsing_started = true;
                }
                _ if c.is_numeric() => {
                    if c == '0' {
                        if is_first_nonzero_digit {
                            int.push(c);
                            is_parsing_started = true;
                        }
                    } else {
                        int.push(c);
                        is_first_nonzero_digit = true;
                        is_parsing_started = true;
                    }
                }
                '.' => break,
                _ => return 0,
            }
        }
        if int.len() == 0 {
            return 0;
        }
        let max = if is_num_positive { i32::MAX } else { i32::MIN };
        let max_chars: Vec<char> = max
            .to_string()
            .chars()
            .skip_while(|c| !c.is_numeric())
            .collect();
        if int.len() > max_chars.len() {
            return max;
        }
        if int.len() == max_chars.len() {
            let mut is_equal = true;
            for (x, y) in int.iter().zip(max_chars.iter()) {
                if *x < *y {
                    is_equal = false;
                    break;
                }
                if *x > *y {
                    return max;
                }
            }
            if is_equal {
                return max;
            }
        }
        let ans = int
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i as u32, c.to_digit(10).unwrap()))
            .fold(0i32, |acc, (i, d)| acc + 10i32.pow(i) * d as i32);
        if is_num_positive {
            ans
        } else {
            -ans
        }
    }
}

fn main() {
    assert_eq!(-1, Solution::my_atoi("-000000000000001".to_string()));
    assert_eq!(0, Solution::my_atoi("-.1".to_string()));
    assert_eq!(3, Solution::my_atoi("3.14".to_string()));
    assert_eq!(0, Solution::my_atoi("00000-42a1234".to_string()));
    assert_eq!(
        12345678,
        Solution::my_atoi("  0000000000012345678".to_string())
    );
    assert_eq!(1, Solution::my_atoi("+1".to_string()));
    assert_eq!(i32::MAX, Solution::my_atoi(i32::MAX.to_string()));
    assert_eq!(i32::MIN, Solution::my_atoi(i32::MIN.to_string()));
    assert_eq!(42, Solution::my_atoi("42".to_string()));
    assert_eq!(-42, Solution::my_atoi("  -42".to_string()));
    assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
    assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
    assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
}
