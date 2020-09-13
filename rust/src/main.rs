mod helper;

struct Solution;

impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        assert!(num >= 0);
        let num = num.to_string();
        let chars: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let len = chars.len();
        let mut dp = vec![0; len + 1];
        dp[1] = 1;
        for i in 2..=len {
            dp[i] = dp[i - 1];
            let x = chars[i - 2] * 10 + chars[i - 1];
            if x <= 25 && x >= 10 {
                dp[i] += std::cmp::max(dp[i - 2], 1);
            }
        }
        dp[len]
    }
}

fn main() {
    assert_eq!(2, Solution::translate_num(18580));
    assert_eq!(2, Solution::translate_num(25));
    assert_eq!(1, Solution::translate_num(0));
    assert_eq!(5, Solution::translate_num(12258));
}
