mod helper;

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let nums = &mut { nums };
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() as i32 - 1;
        while i <= j {
            let mid = (j - i) / 2 + i;
            if nums[mid as usize] == mid {
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }
        i
    }
}

fn main() {
    assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
    assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
}
