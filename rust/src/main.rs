mod helper;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let n = prices.len();
        let mut dp = vec![vec![0, 0]; n + 1];
        dp[0][0] = 0;
        dp[0][1] = std::i32::MIN;
        dp[1][0] = 0;
        dp[1][1] = -prices[0];
        for x in 2..n + 1 {
            dp[x][0] = std::cmp::max(dp[x - 1][0], dp[x - 1][1] + prices[x - 1]);
            dp[x][1] = std::cmp::max(dp[x - 2][0] - prices[x - 1], dp[x - 1][1]);
        }
        dp[n][0]
    }
}

fn main() {
    assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]));
}
