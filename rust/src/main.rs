mod helper;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut dp = vec![0; len + 1];
        dp[1] = nums[0];
        for i in 2..=len {
            dp[i] = std::cmp::max(dp[i - 2] + nums[i - 1], dp[i - 1]);
        }
        dp[len]
    }
}

fn main() {
    assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
}
