mod helper;

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        assert!(nums.len() >= 1);
        use std::collections::HashSet;
        fn permute(nums: Vec<i32>, i: usize, rs: &mut HashSet<Vec<i32>>) {
            if i == nums.len() {
                rs.insert(nums);
                return;
            }
            for j in i..nums.len() {
                let mut nums = nums.clone();
                nums.swap(i, j);
                permute(nums, i + 1, rs);
            }
        }
        let mut rs = HashSet::new();
        permute(nums, 0, &mut rs);
        rs.into_iter().collect()
    }
}

fn main() {
    println!("{:?}", Solution::permute_unique(vec![1, 2, 3]));
    println!("{:?}", Solution::permute_unique(vec![1, 1, 2]));
}
