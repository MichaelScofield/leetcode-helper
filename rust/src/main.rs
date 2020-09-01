mod helper;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let n = nums.len();
        let mut dp = vec![0; n];
        let mut max = std::i32::MIN;
        for i in 1..=n {
            for j in 0..=n - i {
                dp[j] = dp[j] + nums[j + i - 1];
                max = std::cmp::max(max, dp[j]);
            }
        }
        max
    }
}

fn main() {
    assert_eq!(6, Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    assert_eq!(-1, Solution::max_sub_array(vec![-1]));
}
