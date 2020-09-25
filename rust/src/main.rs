mod helper;

struct Solution;

impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        assert_eq!(nums.len(), 5);
        let nums = &mut { nums };
        nums.sort();
        let jokers = nums.iter().filter(|&&c| c == 0).count();
        let mut gaps = 0;
        let mut i = jokers;
        while i + 1 < nums.len() {
            let x = nums[i + 1] - nums[i];
            if x == 0 {
                return false;
            }
            if x > 1 {
                gaps += x - 1;
            }
            i += 1;
        }
        jokers >= gaps as usize
    }
}

fn main() {
    assert!(Solution::is_straight(vec![1, 2, 3, 4, 5]));
    assert!(Solution::is_straight(vec![0, 0, 1, 2, 5]));
    assert!(!Solution::is_straight(vec![0, 0, 1, 2, 6]));
    assert!(Solution::is_straight(vec![0, 0, 0, 0, 0]));
    assert!(Solution::is_straight(vec![0, 0, 0, 0, 1]));
    assert!(Solution::is_straight(vec![11, 0, 9, 0, 0]));
}
