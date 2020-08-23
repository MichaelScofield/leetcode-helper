mod helper;

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(nums.len() > 0);
        if nums.len() == 1 {
            return nums[0];
        }
        let n = nums.len();
        if nums[0] < nums[n - 1] {
            return nums[0];
        }
        let mut i = 0;
        let mut j = n - 1;
        loop {
            let mid = i + (j - i) / 2;
            if nums[mid] > nums[i] {
                i = mid;
            } else if nums[mid] < nums[j] {
                j = mid;
            } else {
                for k in i..j {
                    if nums[k] > nums[k + 1] {
                        return nums[k + 1];
                    }
                }
                break;
            }
            if i + 1 >= j {
                break;
            }
        }
        nums[j]
    }
}

fn main() {
    println!("{}", Solution::find_min(vec![3, 4, 5, 1, 2]));
    println!("{}", Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    println!("{}", Solution::find_min(vec![1, 1, 1]));
    println!("{}", Solution::find_min(vec![1, 1, 2]));
    println!("{}", Solution::find_min(vec![1, 1, 1, 0, 1]));
}
