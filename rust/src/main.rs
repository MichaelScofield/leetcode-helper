mod helper;

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        fn can_jump(nums: &[usize]) -> bool {
            if nums[0] >= nums.len() - 1 {
                return true;
            }
            for j in 1..=nums[0] {
                if can_jump(&nums[j..]) {
                    return true;
                }
            }
            false
        }
        let nums: Vec<usize> = nums.iter().map(|&i| i as usize).collect();
        can_jump(nums.as_slice())
    }
}

fn main() {
    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    assert!(!Solution::can_jump(vec![0, 2, 1, 1, 4]));
    assert!(Solution::can_jump(vec![1]));
    assert!(Solution::can_jump(vec![1, 2]));
}
