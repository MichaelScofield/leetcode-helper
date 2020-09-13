mod helper;

struct Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        assert!(m > 0);
        let n = grid[0].len();
        assert!(n > 0);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                let value = grid[i - 1][j - 1];
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]) + value;
            }
        }
        dp[m][n]
    }
}

fn main() {
    assert_eq!(12, Solution::max_value(vecvec![[1,3,1],[1,5,1],[4,2,1]]));
}
