mod helper;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let n = nums.len();
        // dp[i][0]: i nums, max sub array ends in nums[i]
        // dp[i][1]: i nums, max sub array NOT ends in nums[i]
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = nums[0];
        dp[0][1] = std::i32::MIN;
        for i in 1..n {
            let num = nums[i];
            dp[i][0] = std::cmp::max(dp[i - 1][0] + num, num);
            dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]);
        }
        std::cmp::max(dp[n - 1][0], dp[n - 1][1])
    }
}

fn main() {
    assert_eq!(6, Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    assert_eq!(-1, Solution::max_sub_array(vec![-1]));
}
