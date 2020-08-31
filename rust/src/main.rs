mod helper;

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let n = nums.len();
        let sum = nums.iter().sum::<i32>() as usize;

        let s = s.abs() as usize;
        if sum < s {
            return 0;
        }

        // State transition equation:
        // dp[i][j] = dp[i - 1][|j - nums[i]|] + dp[i - 1][j + nums[i]]
        // Why "|j - nums[i]|"?
        // Because for every solution,
        // Σ(nums) = -s <=> -Σ(nums) = s,
        // we can omit the storage for negative sum by dp[-sum] = dp[sum].
        let mut dp = vec![vec![0; sum + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            let num = nums[i - 1] as usize;
            for j in 0..=sum {
                if j >= num {
                    dp[i][j] += dp[i - 1][j - num];
                } else {
                    dp[i][j] += dp[i - 1][num - j];
                }
                if j + num <= sum {
                    dp[i][j] += dp[i - 1][j + num];
                }
            }
        }
        dp[n][s]
    }
}

fn main() {
    assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    assert_eq!(0, Solution::find_target_sum_ways(vec![1], 2));
}
