mod helper;

struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        assert!(len >= 1);
        let mut sum = 0;
        for l in (1..=len).step_by(2) {
            let mut i = 0;
            let mut j = i + l;
            while j <= len {
                sum += arr[i..j].iter().sum::<i32>();
                i += 1;
                j += 1;
            }
        }
        sum
    }
}

fn main() {
    assert_eq!(58, Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]));
    assert_eq!(3, Solution::sum_odd_length_subarrays(vec![1, 2]));
    assert_eq!(66, Solution::sum_odd_length_subarrays(vec![10, 11, 12]));
}
