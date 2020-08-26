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
        for x in 1..n + 1 {
            dp[x][0] = std::cmp::max(dp[x - 1][0], dp[x - 1][1] + prices[x - 1]);
            dp[x][1] = std::cmp::max(dp[x - 1][0] - prices[x - 1], dp[x - 1][1]);
        }
        dp[n][0]
    }
}

fn main() {
    assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
