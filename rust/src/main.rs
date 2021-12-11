mod helper;

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        let nums = &mut { nums };

        let mut i = 0;
        while i < nums.len() {
            if nums[i] > 0 {
                let j = (nums[i] - 1) as usize;
                if j < nums.len() && j as i32 != nums[j] - 1 {
                    let t = nums[j];
                    nums[j] = nums[i];
                    nums[i] = t;
                    continue;
                }
            }

            i += 1;
        }

        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
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
