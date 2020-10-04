mod helper;

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        assert!(len > 0);
        let mut dp = vec![std::i32::MAX; len];
        dp[0] = 0;
        for i in 1..len {
            for j in 0..i {
                if nums[j] >= (i - j) as i32 {
                    dp[i] = std::cmp::min(dp[i], dp[j] + 1);
                }
            }
        }
        dp[len - 1]
    }
}

fn main() {
    assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
}
