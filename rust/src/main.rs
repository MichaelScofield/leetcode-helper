mod helper;

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        if len == 0 {
            return Vec::with_capacity(0);
        }
        let intervals = &mut { intervals };
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut merged = vec![];
        let mut last_merge = vec![intervals[0][0], intervals[0][1]];
        for i in 1..len {
            let interval = &intervals[i];
            if interval[0] <= last_merge[1] {
                last_merge = vec![last_merge[0], std::cmp::max(last_merge[1], interval[1])];
            } else {
                merged.push(last_merge);
                last_merge = vec![interval[0], interval[1]];
            }
        }
        merged.push(last_merge);
        merged
    }
}

fn main() {
    println!("{:?}", Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
    println!("{:?}", Solution::merge(vec![vec![1, 4], vec![4, 5]]));
    println!("{:?}", Solution::merge(vec![vec![4, 5], vec![1, 4]]));
    println!("{:?}", Solution::merge(vec![vec![1, 4], vec![5, 6]]));
    println!("{:?}", Solution::merge(vec![vec![5, 6], vec![1, 4]]));
    println!("{:?}", Solution::merge(vec![vec![1, 6], vec![1, 4], vec![2, 5]]));
    println!("{:?}", Solution::merge(vec![vec![1, 6]]));
}
