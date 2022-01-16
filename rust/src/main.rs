mod helper;

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        let mut min_len = i32::MAX;
        let mut l = 0;
        let mut r = 0;
        let mut sum = nums[0];
        loop {
            if sum >= target {
                min_len = std::cmp::min((r - l + 1) as i32, min_len);
                sum -= nums[l];
                l += 1;
            } else {
                r += 1;
                if r == nums.len() {
                    break;
                }
                sum += nums[r];
            }
        }
        if min_len == i32::MAX {
            0
        } else {
            min_len
        }
    }
}

fn main() {
    assert_eq!(1, Solution::min_sub_array_len(1, vec![1]));
    assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    assert_eq!(1, Solution::min_sub_array_len(4, vec![1, 4, 4]));
    assert_eq!(
        0,
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
    );
}
