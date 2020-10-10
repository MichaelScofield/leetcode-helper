mod helper;

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![vec![0; len + 2]; len + 2];
        for j in 1..=(len + 1) {
            for i in (0..j).rev() {
                for k in (i + 1)..j {
                    let curr = nums[k - 1];
                    let left = if i == 0 { 1 } else { nums[i - 1] };
                    let right = if j == len + 1 { 1 } else { nums[j - 1] };
                    dp[i][j] = std::cmp::max(dp[i][j],
                                             dp[i][k] + dp[k][j] + left * curr * right);
                }
            }
        }
        dp[0][len + 1]
    }
}

fn main() {
    assert_eq!(167, Solution::max_coins(vec![3, 1, 5, 8]));
}
