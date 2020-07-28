mod helper;

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 1 {
            return Vec::with_capacity(0);
        }
        fn permute(nums: Vec<i32>, i: usize, rs: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                rs.push(nums);
                return;
            }
            for j in i..nums.len() {
                let mut nums = nums.clone();
                nums.swap(i, j);
                permute(nums, i + 1, rs);
            }
        }
        let mut rs = vec![];
        permute(nums, 0, &mut rs);
        rs
    }
}

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
    println!("{:?}", Solution::permute(vec![1]));
    println!("{:?}", Solution::permute(vec![1, 2]));
    println!("{:?}", Solution::permute(vec![1, 2, 3, 4]));
}
