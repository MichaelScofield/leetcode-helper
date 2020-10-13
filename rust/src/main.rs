mod helper;

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![1; len];
        for i in 0..len {
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap_or(0)
    }
}

fn main() {
    assert_eq!(6, Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]));
    assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    assert_eq!(1, Solution::length_of_lis(vec![10, 9]));
    assert_eq!(1, Solution::length_of_lis(vec![10]));
    assert_eq!(0, Solution::length_of_lis(vec![]));
}
