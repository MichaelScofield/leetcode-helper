use crate::helper::util::to_string_vec;

mod helper;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return Vec::with_capacity(0);
        }
        let prime_nums = vec![
            2, 3, 5, 7, 11,
            13, 17, 19, 23, 29,
            31, 37, 41, 43, 47,
            53, 59, 61, 67, 71,
            73, 79, 83, 89, 97,
            101];
        let calc_primes = |str: &str| -> u64 {
            str.bytes().map(|c| prime_nums[(c - b'a') as usize]).product()
        };
        use std::collections::HashMap;
        let mut groups = HashMap::new();
        for str in strs {
            let m = calc_primes(&str);
            groups.entry(m).or_insert_with(|| Vec::new()).push(str);
        }
        groups.into_iter().map(|e| e.1).collect()
    }
}

fn main() {
    let strs = to_string_vec(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
    println!("{:?}", Solution::group_anagrams(strs));
    let strs = to_string_vec(vec!["aaa", "aaa", "bb", "bb", "a", "a"]);
    println!("{:?}", Solution::group_anagrams(strs));
    let strs = to_string_vec(vec!["",]);
    println!("{:?}", Solution::group_anagrams(strs));
}
