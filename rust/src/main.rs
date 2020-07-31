mod helper;

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut paths = vec![vec![0; m as usize]; n as usize];
        paths[0][0] = 1;
        fn calc_paths(m: i32, n: i32, paths: &mut Vec<Vec<i32>>) -> i32 {
            let y = n as usize - 1;
            let x = m as usize - 1;
            if paths[y][x] == 0 {
                paths[y][x] =
                    if m > 1 {
                        calc_paths(m - 1, n, paths)
                    } else {
                        0
                    } +
                    if n > 1 {
                        calc_paths(m, n - 1, paths)
                    } else {
                        0
                    }
            }
            paths[y][x]
        }
        calc_paths(m, n, &mut paths)
    }
}

fn main() {
    assert_eq!(3, Solution::unique_paths(3, 2));
    assert_eq!(28, Solution::unique_paths(7, 3));
    assert_eq!(1, Solution::unique_paths(1, 1));
    assert_eq!(1, Solution::unique_paths(1, 2));
    assert_eq!(1, Solution::unique_paths(2, 1));
    assert_eq!(2, Solution::unique_paths(2, 2));
}
