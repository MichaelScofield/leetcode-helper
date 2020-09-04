mod helper;

struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        assert!(piles.len() >= 2);
        use std::collections::HashMap;
        let mut memo = HashMap::new();
        fn stone_game((i, j): (usize, usize), memo: &mut HashMap<(usize, usize), bool>) -> bool {
            if j - i == 1 {
                return true;
            }
            if let Some(&win) = memo.get(&(i, j)) {
                return win;
            }
            let win = !stone_game((i + 1, j), memo)
                || !stone_game((i, j - 1), memo);
            memo.insert((i, j), win);
            win
        }
        stone_game((0, piles.len() - 1), &mut memo)
    }
}

fn main() {
    assert!(Solution::stone_game(vec![5, 3, 4, 5]));
}
