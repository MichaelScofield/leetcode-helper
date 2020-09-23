mod helper;

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        assert!(len > 0);
        let mut pre_sum = vec![0; len + 1];
        for i in 0..len {
            pre_sum[i + 1] = pre_sum[i] + nums[i];
        }
        let mut subarrays = 0;
        for w in 1..=len {
            for i in 0..len - w + 1 {
                if pre_sum[i + w] - pre_sum[i] == k {
                    subarrays += 1;
                }
            }
        }
        subarrays
    }
}

fn main() {
    assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
}
