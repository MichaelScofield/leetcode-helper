mod helper;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let mut max_profit = 0;
        let mut min_price = std::i32::MAX;
        for price in prices {
            if price < min_price {
                min_price = price;
            } else {
                max_profit = std::cmp::max(max_profit, price - min_price);
            }
        }
        max_profit
    }
}

fn main() {
    assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
