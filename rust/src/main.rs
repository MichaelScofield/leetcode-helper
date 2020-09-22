mod helper;

struct Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        assert!(target > 0);
        let mut i = 1;
        let mut j = 2;
        let mut sequence = vec![];
        loop {
            let sum = (i + j) * (j - i + 1) / 2;
            if sum < target {
                j += 1;
            } else {
                if sum == target {
                    sequence.push((i..=j).collect());
                }
                i += 1;
            }
            if i * 2 >= target {
                break;
            }
        }
        sequence
    }
}

fn main() {
    assert_eq!(vecvec![[2,3,4],[4,5]], Solution::find_continuous_sequence(9));
    assert_eq!(vecvec![[1,2,3,4,5],[4,5,6],[7,8]], Solution::find_continuous_sequence(15));
}
