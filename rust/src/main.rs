mod helper;

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.len() < 1 {
            return -1;
        }
        if amount < 1 {
            return 0;
        }
        fn coin_change(coins: &Vec<i32>, amount: i32, n: i32, min: &mut i32) {
            for &coin in coins {
                if amount == coin {
                    if n + 1 < *min {
                        *min = n + 1;
                    }
                    return;
                }
                if amount > coin {
                    coin_change(&coins, amount - coin, n + 1, min);
                }
            }
        }
        let mut min = std::i32::MAX;
        coin_change(&coins, amount, 0, &mut min);
        if min == std::i32::MAX { -1 } else { min }
    }
}

fn main() {
    assert_eq!(20, Solution::coin_change(vec![186, 419, 83, 408], 6249));
    assert_eq!(20, Solution::coin_change(vec![1, 2, 5], 100));
    assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
    assert_eq!(4, Solution::coin_change(vec![1, 2, 5], 20));
    assert_eq!(1, Solution::coin_change(vec![1, 2, 5], 1));
    assert_eq!(1, Solution::coin_change(vec![1, 2, 5], 2));
    assert_eq!(-1, Solution::coin_change(vec![2], 3));
}
