mod helper;

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 != 0 {
            return false;
        }
        let n = nums.len();
        let w = sum / 2;
        let mut dp = vec![vec![false; w + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = true;
        }
        for i in 1..=n {
            let num = nums[i - 1] as usize;
            for j in num..=w {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - num];
            }
        }
        dp[n][w]
    }
}

fn main() {
    assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
}
