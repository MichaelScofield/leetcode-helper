mod helper;

struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let points = &mut { points };
        let n = points.len();
        if n <= 1 {
            return n as i32;
        }

        points.sort_by(|a, b| {
            assert!(a.len() == 2 && b.len() == 2 && a[1] > a[0] && b[1] > b[0]);
            a[1].cmp(&b[1])
        });

        let mut arrows = 1;
        let mut i = 0;
        let mut j = 1;
        while j < n {
            if points[i][1] < points[j][0] {
                i = j;
                arrows += 1;
            }
            j += 1;
        }
        arrows
    }
}

fn main() {
    let points = vecvec![[10,16], [2,8], [1,6], [7,12]];
    assert_eq!(2, Solution::find_min_arrow_shots(points));
    let points = vecvec![[1,2],[2,3],[3,4],[4,5]];
    assert_eq!(2, Solution::find_min_arrow_shots(points));
}
