mod helper;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        use std::collections::HashSet;
        let mut set = HashSet::new();
        for num in nums {
            set.insert(num);
        }

        let mut sequence_len = 1;
        let mut longest_sequence_len = 1;
        for &num in set.iter() {
            if !set.contains(&(num - 1)) {
                sequence_len = 1;
                let mut i = num + 1;
                while set.contains(&i) {
                    sequence_len += 1;
                    i += 1;
                }
                longest_sequence_len = std::cmp::max(longest_sequence_len, sequence_len);
            }
        }
        std::cmp::max(longest_sequence_len, sequence_len)
    }
}

fn main() {
    assert_eq!(
        10,
        Solution::longest_consecutive(vec![
            0, 1, 2, 4, 8, 5, 6, 7, 9, 3, 55, 88, 77, 99, 999999999
        ])
    );
    assert_eq!(1, Solution::longest_consecutive(vec![1]));
    assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    assert_eq!(
        9,
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
    );
}
