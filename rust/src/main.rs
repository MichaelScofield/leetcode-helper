mod helper;

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 1 {
            return Vec::with_capacity(0);
        }
        fn subsets(nums: &[i32], mut result: Vec<i32>, results: &mut Vec<Vec<i32>>) {
            results.push(result.clone());
            for i in 0..nums.len() {
                result.push(nums[i]);
                subsets(&nums[i + 1..], result.clone(), results);
                result.pop();
            }
        }
        let mut results = vec![];
        subsets(nums.as_slice(), vec![], &mut results);
        results
    }
}

fn main() {
    println!("{:?}", Solution::subsets(vec![]));
    println!("{:?}", Solution::subsets(vec![1]));
    println!("{:?}", Solution::subsets(vec![1, 2]));
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
    println!("{:?}", Solution::subsets(vec![1, 2, 3, 4]));
}
