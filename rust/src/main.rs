mod helper;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        assert!(nums.len() >= 2);
        let nums = &mut { nums };

        fn xor(nums: &Vec<i32>, i: usize, j: usize) -> i32 {
            let mut xor = nums[i];
            for k in i + 1..j {
                xor ^= nums[k];
            }
            xor
        }

        let x = xor(nums, 0, nums.len());
        let mut bit_mask = 1;
        for i in 0..32 {
            bit_mask = 1 << i;
            if x & bit_mask == bit_mask {
                break;
            }
        }
        assert_ne!(bit_mask, 0);

        let mut i = 0;
        let mut j = nums.len() - 1;
        while i <= j {
            if &nums[i] & bit_mask == bit_mask {
                nums.swap(i, j);
                j -= 1;
            } else {
                i += 1;
            }
        }
        let p = j + 1;
        vec![xor(nums, 0, p), xor(nums, p, nums.len())]
    }
}

fn main() {
    assert_eq!(vec![5, 3], Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
    assert_eq!(vec![0, -1], Solution::single_number(vec![-1, 0]));
    assert_eq!(vec![0, 1], Solution::single_number(vec![0, 1]));
}
