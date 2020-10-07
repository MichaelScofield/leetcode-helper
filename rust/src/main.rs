mod helper;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            return nums[0];
        }
        fn rob(nums: &[i32]) -> i32 {
            let len = nums.len();
            let mut dp = vec![0; len + 1];
            dp[1] = nums[0];
            for i in 2..=len {
                dp[i] = std::cmp::max(dp[i - 2] + nums[i - 1], dp[i - 1]);
            }
            dp[len]
        }
        std::cmp::max(rob(&nums[0..len - 1]), rob(&nums[1..len]))
    }
}

fn main() {
    assert_eq!(2, Solution::rob(vec![2]));
    assert_eq!(2, Solution::rob(vec![1, 2]));
    assert_eq!(100, Solution::rob(vec![1, 2, 100]));
    assert_eq!(3, Solution::rob(vec![2, 3, 2]));
    assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
}
