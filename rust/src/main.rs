mod helper;

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        assert!(intervals.len() > 0);
        if intervals.len() == 1 {
            return 1;
        }
        let intervals = &mut { intervals };
        use std::cmp::Ordering;
        intervals.sort_by(|a, b| {
            let ordering = a[0].cmp(&b[0]);
            if ordering == Ordering::Equal {
                b[1].cmp(&a[1])
            } else {
                ordering
            }
        });
        let mut max = &intervals[0];
        let mut remain = 1;
        for i in 1..intervals.len() {
            let curr = &intervals[i];
            if curr[1] > max[1] {
                remain += 1;
                max = curr;
            }
        }
        remain
    }
}

fn main() {
    assert_eq!(2, Solution::remove_covered_intervals(vecvec![[1,4],[3,6],[2,8]]));
}
