mod helper;

struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        assert!(n >= 0 && n <= 100);
        if n == 0 || n == 1 {
            return n;
        }
        let n = n as usize;
        let mut nums = vec![0; n + 1];
        nums[0] = 0;
        nums[1] = 1;
        for i in 2..=n {
            if i % 2 == 0 {
                nums[i] = nums[i / 2];
            } else {
                nums[i] = nums[(i - 1) / 2] + nums[(i - 1) / 2 + 1];
            }
        }
        nums.sort();
        nums[nums.len() - 1]
    }
}

fn main() {
    assert_eq!(3, Solution::get_maximum_generated(7));
    assert_eq!(1, Solution::get_maximum_generated(2));
    assert_eq!(2, Solution::get_maximum_generated(3));
}
