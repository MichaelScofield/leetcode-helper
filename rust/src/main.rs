mod helper;

struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let intervals = &mut { intervals };
        let n = intervals.len();
        if n <= 1 {
            return 0;
        }

        use std::cmp::Ordering;
        intervals.sort_by(|a, b| {
            assert!(a.len() == 2 && b.len() == 2 && a[1] > a[0] && b[1] > b[0]);
            match a[1].cmp(&b[1]) {
                Ordering::Equal => a[0].cmp(&b[0]),
                ordering => ordering
            }
        });

        let mut erased = 0;
        let mut i = 0;
        let mut j = 1;
        while j < n {
            if intervals[i][1] <= intervals[j][0] {
                i = j;
            } else {
                erased += 1;
            }
            j += 1;
        }
        erased
    }
}

fn main() {
    let intervals = vecvec![[1,2],[2,3],[3,4],[1,3]];
    assert_eq!(1, Solution::erase_overlap_intervals(intervals));
    let intervals = vecvec![[1,2],[1,2],[1,2]];
    assert_eq!(2, Solution::erase_overlap_intervals(intervals));
    let intervals = vecvec![[1,2],[2,3]];
    assert_eq!(0, Solution::erase_overlap_intervals(intervals));
    let intervals = vecvec![[1,9],[1,5],[5,9]];
    assert_eq!(1, Solution::erase_overlap_intervals(intervals));
    let intervals = vecvec![[0,2],[1,3],[2,4],[3,5],[4,6]];
    assert_eq!(2, Solution::erase_overlap_intervals(intervals));
    let intervals = vecvec![[0,100],[99,101],[100,200]];
    assert_eq!(1, Solution::erase_overlap_intervals(intervals));
}
