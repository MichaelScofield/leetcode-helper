mod helper;

struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let n = prices.len();
        let k = k as usize;
        if k > n / 2 {
            return Solution::max_profit_simple(prices);
        }
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

    fn max_profit_simple(prices: Vec<i32>) -> i32 {
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
    assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
    assert_eq!(7, Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
}
