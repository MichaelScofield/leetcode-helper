mod helper;

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.len() >= 3);

        let nums = &mut { nums };
        nums.sort();

        let mut closest: i64 = i32::MAX as i64;
        let target = target as i64;
        let mut i = 0;
        while i < nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    j += 1;
                    continue;
                }
                if k < nums.len() - 1 && nums[k] == nums[k + 1] {
                    k -= 1;
                    continue;
                }

                let sum = nums[i] as i64 + nums[j] as i64 + nums[k] as i64;
                if sum == target {
                    return sum as i32;
                } else {
                    if sum < target {
                        j += 1;
                    } else {
                        k -= 1;
                    }

                    if (sum - target).abs() < (closest - target).abs() {
                        closest = sum;
                    }
                }
            }
            i += 1;
        }
        closest as i32
    }
}

fn main() {
    assert_eq!(2, Solution::three_sum_closest(vec![-1, 0, 1, 1, 55], 3));
    assert_eq!(-2, Solution::three_sum_closest(vec![-3, -2, -5, 3, -4], -1));
    assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0], 1))
}
