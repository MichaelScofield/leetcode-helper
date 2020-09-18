mod helper;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        assert!(nums.len() > 0);
        let mut bits_count = vec![0; 32];
        for num in nums {
            for i in 0..32 {
                if num & (1 << i) != 0 {
                    bits_count[i] += 1;
                }
            }
        }

        let mut n = 0;
        for i in 0..32 {
            let p = bits_count[i] % 3;
            if p == 1 {
                n += 2i32.pow(i as u32);
            }
        }
        n
    }
}

fn main() {
    assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    assert_eq!(1, Solution::single_number(vec![1]));
}
