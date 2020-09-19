mod helper;

struct Solution;

impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        assert!(nums.len() >= 1 && requests.len() >= 1);
        let mut buckets = vec![0; nums.len()];
        for request in requests {
            let start = request[0];
            let end = request[1];
            for i in start..=end {
                buckets[i as usize] += 1;
            }
        }
        buckets.sort();

        let nums = &mut { nums };
        nums.sort();
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i] * buckets[i];
        }
        sum
    }
}

fn main() {
    let requests = vecvec![[1,3],[0,1]];
    assert_eq!(19, Solution::max_sum_range_query(vec![1, 2, 3, 4, 5], requests));
    let requests = vecvec![[0,1]];
    assert_eq!(11, Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 6], requests));
    let requests = vecvec![[0,2],[1,3],[1,1]];
    assert_eq!(47, Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 10], requests));
    let requests = vecvec![[0,0],[0,0]];
    assert_eq!(2, Solution::max_sum_range_query(vec![1], requests));
}
