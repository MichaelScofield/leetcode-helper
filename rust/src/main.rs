mod helper;

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let len = nums.len();
        assert!(len >= 1 && p >= 1);
        let nums = nums.iter().map(|&n| n as u64).collect::<Vec<u64>>();
        let p = p as u64;
        let sum = nums.iter().sum::<u64>();
        if sum % p == 0 {
            return 0;
        }
        for l in 1..len {
            let mut i = 0;
            let mut j = i + l;
            let mut subarray_sum = nums[i..j].iter().sum::<u64>();
            loop {
                if (sum - subarray_sum) % p == 0 {
                    return l as i32;
                }
                if j == len {
                    break;
                }
                subarray_sum -= nums[i];
                subarray_sum += nums[j];
                i += 1;
                j += 1;
            }
        }
        -1
    }
}

fn main() {
    assert_eq!(1, Solution::min_subarray(vec![3, 1, 4, 2], 6));
    assert_eq!(2, Solution::min_subarray(vec![6, 3, 5, 2], 9));
    assert_eq!(0, Solution::min_subarray(vec![1, 2, 3], 3));
    assert_eq!(-1, Solution::min_subarray(vec![1, 2, 3], 7));
    assert_eq!(0, Solution::min_subarray(vec![1000000000, 1000000000, 1000000000], 3));
}
