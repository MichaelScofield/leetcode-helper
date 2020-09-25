mod helper;

struct Solution;

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if a.len() == 0 || b.len() == 0 {
            return vec![];
        }
        let mut intersections = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < a.len() && j < b.len() {
            let a = &a[i];
            let b = &b[j];
            if b[0] > a[1] {
                i += 1;
            } else if a[0] > b[1] {
                j += 1;
            } else {
                intersections.push(vec![std::cmp::max(a[0], b[0]), std::cmp::min(a[1], b[1])]);
                if a[1] < b[1] {
                    i += 1;
                } else {
                    j += 1;
                }
            }
        }
        intersections
    }
}

fn main() {
    let a = vecvec![[0,2],[5,10],[13,23],[24,25]];
    let b = vecvec![[1,5],[8,12],[15,24],[25,26]];
    assert_eq!(vecvec![[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]], Solution::interval_intersection(a, b));
}
