mod helper;

struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        assert!(dungeon.len() >= 1);
        let m = dungeon.len();
        assert!(dungeon[0].len() >= 1);
        let n = dungeon[0].len();

        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[m - 1][n] = 1;
        dp[m][n - 1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                dp[i][j] =
                    std::cmp::max(std::cmp::min(dp[i][j + 1], dp[i + 1][j]) - dungeon[i][j], 1);
            }
        }
        dp[0][0]
    }
}

fn main() {
    assert_eq!(4, Solution::calculate_minimum_hp(vecvec![[-3, 5]]));
    assert_eq!(1, Solution::calculate_minimum_hp(vecvec![[0, 0]]));
    assert_eq!(
        7,
        Solution::calculate_minimum_hp(vecvec![[-2, -3, 3], [-5, -10, 1], [10, 30, -5]])
    );
}
