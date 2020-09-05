mod helper;

struct Solution;

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return nums;
        }
        let mut i = 0;
        let mut j = nums.len() - 1;
        let nums = &mut { nums };
        while i < j {
            while i < j {
                if nums[i] % 2 == 0 {
                    break;
                }
                i += 1;
            }
            while i < j {
                if nums[j] % 2 == 1 {
                    break;
                }
                j -= 1;
            }
            nums.swap(i, j);
        }
        nums.to_owned()
    }
}

fn main() {
    println!("{:?}", Solution::exchange(vec![1, 2, 3, 4]));
}
