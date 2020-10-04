mod helper;

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        assert!(len > 0);
        let mut dp = vec![false; len];
        dp[0] = true;
        for i in 1..len {
            for j in 0..i {
                if dp[j] && nums[j] >= (i - j) as i32 {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[len - 1]
    }
}

fn main() {
    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    assert!(!Solution::can_jump(vec![0, 2, 1, 1, 4]));
    assert!(Solution::can_jump(vec![1]));
    assert!(Solution::can_jump(vec![1, 2]));
}
