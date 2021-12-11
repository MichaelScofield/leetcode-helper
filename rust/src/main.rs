mod helper;

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        let nums = &mut { nums };

        for num in nums.iter_mut() {
            if *num <= 0 {
                *num = i32::MAX;
            }
        }

        for i in 0..nums.len() {
            if nums[i] == i32::MAX {
                continue;
            }

            let j = (nums[i].abs() - 1) as usize;
            if j < nums.len() {
                if nums[j] > 0 {
                    nums[j] = -nums[j];
                }
            }
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                return (i + 1) as i32;
            }
        }
        (nums.len() + 1) as i32
    }
}

fn main() {
    assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    assert_eq!(2, Solution::first_missing_positive(vec![1, 1]));
    assert_eq!(4, Solution::first_missing_positive(vec![1, 2, 3]));
    assert_eq!(1, Solution::first_missing_positive(vec![4, 2, 3]));
    assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
}
