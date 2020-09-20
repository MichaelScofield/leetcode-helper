mod helper;

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        assert!(grid.len() > 0 && grid[0].len() > 0);
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        for i in 0..rows {
            dp[i][0] = grid[i][0] as i64;
        }
        for i in 0..cols {
            dp[0][i] = grid[0][i] as i64;
        }
        for i in 1..=rows {
            for j in 1..=cols {
                let num = grid[i - 1][j - 1] as i64;
                dp[i][j] = std::cmp::max(dp[i - 1][j] * num, dp[i][j - 1] * num);
            }
        }
        if dp[rows][cols] < 0 {
            -1
        } else {
            (dp[rows][cols] % 1_000_000_007) as i32
        }
    }
}

fn main() {
    assert_eq!(-1, Solution::max_product_path(vecvec![[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]));
    assert_eq!(8, Solution::max_product_path(vecvec![[1,-2,1],[1,-2,1],[3,-4,1]]));
    assert_eq!(0, Solution::max_product_path(vecvec![[1, 3],[0,-4]]));
    assert_eq!(2, Solution::max_product_path(vecvec![[ 1, 4,4,0],[-2, 0,0,1],[ 1,-1,1,1]]));
}
