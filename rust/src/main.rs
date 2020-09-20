mod helper;

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        assert!(grid.len() > 0 && grid[0].len() > 0);
        let rows = grid.len();
        let cols = grid[0].len();
        let mut max = vec![vec![0; cols]; rows];
        let mut min = vec![vec![0; cols]; rows];
        max[0][0] = grid[0][0] as i64;
        min[0][0] = grid[0][0] as i64;
        for i in 1..rows {
            max[i][0] = max[i - 1][0] * grid[i][0] as i64;
            min[i][0] = min[i - 1][0] * grid[i][0] as i64;
        }
        for j in 1..cols {
            max[0][j] = max[0][j - 1] * grid[0][j] as i64;
            min[0][j] = min[0][j - 1] * grid[0][j] as i64;
        }
        for i in 1..rows {
            for j in 1..cols {
                let num = grid[i][j] as i64;
                let a = max[i - 1][j] * num;
                let b = max[i][j - 1] * num;
                let c = min[i - 1][j] * num;
                let d = min[i][j - 1] * num;
                max[i][j] = std::cmp::max(std::cmp::max(a, b), std::cmp::max(c, d));
                min[i][j] = std::cmp::min(std::cmp::min(a, b), std::cmp::min(c, d));
            }
        }
        if max[rows - 1][cols - 1] < 0 {
            -1
        } else {
            (max[rows - 1][cols - 1] % 1_000_000_007) as i32
        }
    }
}

fn main() {
    assert_eq!(-1, Solution::max_product_path(vecvec![[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]));
    assert_eq!(8, Solution::max_product_path(vecvec![[1,-2,1],[1,-2,1],[3,-4,1]]));
    assert_eq!(0, Solution::max_product_path(vecvec![[1, 3],[0,-4]]));
    assert_eq!(2, Solution::max_product_path(vecvec![[ 1, 4,4,0],[-2, 0,0,1],[ 1,-1,1,1]]));
}
