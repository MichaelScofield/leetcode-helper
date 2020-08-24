use crate::helper::util::to_string_vec;

mod helper;

struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        if target.eq("0000") {
            return 0;
        }
        use std::collections::HashSet;
        let deadends = deadends.into_iter()
            .map(|deadend| deadend.into_bytes().into_iter()
                .map(|b| b - 48)
                .collect::<Vec<u8>>())
            .collect::<HashSet<Vec<u8>>>();
        if deadends.contains(&vec![0, 0, 0, 0]) {
            return -1;
        }
        let target = target.into_bytes().into_iter().map(|b| b - 48).collect::<Vec<u8>>();
        let mut min_turns = std::i32::MAX;
        let mut tried_locks = HashSet::new();
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((vec![0, 0, 0, 0], 0));
        while let Some((mut lock, turns)) = queue.pop_front() {
            if turns >= min_turns {
                continue;
            }
            if tried_locks.contains(&lock) {
                continue;
            }
            tried_locks.insert(lock.clone());
            if lock.eq(&target) {
                min_turns = std::cmp::min(min_turns, turns);
            }
            for i in 0..4 {
                let stash = lock[i];
                for j in vec![1, 9] {
                    lock[i] = (stash + j) % 10;
                    if !deadends.contains(&lock) {
                        queue.push_back((lock.clone(), turns + 1));
                    }
                }
                lock[i] = stash;
            }
        }
        if min_turns == std::i32::MAX { -1 } else { min_turns }
    }
}

fn main() {
    let deadends = to_string_vec(vec!["0201", "0101", "0102", "1212", "2002"]);
    assert_eq!(6, Solution::open_lock(deadends, "0202".to_owned()));
    let deadends = to_string_vec(vec!["8888"]);
    assert_eq!(1, Solution::open_lock(deadends, "0009".to_owned()));
    let deadends = to_string_vec(
        vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"]);
    assert_eq!(-1, Solution::open_lock(deadends, "8888".to_owned()));
    let deadends = to_string_vec(vec!["0000"]);
    assert_eq!(-1, Solution::open_lock(deadends, "8888".to_owned()));
}
