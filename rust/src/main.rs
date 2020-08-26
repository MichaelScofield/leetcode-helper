mod helper;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let n = prices.len();
        let k = 2;
        let mut dp = vec![vec![vec![0, 0]; k + 1]; n + 1];
        for x in 0..n + 1 {
            dp[x][0][0] = 0;
            dp[x][0][1] = std::i32::MIN;
        }
        for x in 0..k + 1 {
            dp[0][x][0] = 0;
            dp[0][x][1] = std::i32::MIN;
        }
        for x in 1..n + 1 {
            for y in 1..k + 1 {
                dp[x][y][0] = std::cmp::max(dp[x - 1][y][0], dp[x - 1][y][1] + prices[x - 1]);
                dp[x][y][1] = std::cmp::max(dp[x - 1][y - 1][0] - prices[x - 1], dp[x - 1][y][1]);
            }
        }
        dp[n][k][0]
    }
}

fn main() {
    assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    assert_eq!(0, Solution::max_profit(vec![1]));
}
