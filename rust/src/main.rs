use crate::helper::util::to_string_vec;

mod helper;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        assert!(strs.len() >= 1);

        let calc_key = |str: &str| -> String {
            let mut bytes = str.as_bytes().to_vec();
            bytes.sort();
            unsafe { String::from_utf8_unchecked(bytes) }
        };

        use std::collections::HashMap;
        let mut groups = HashMap::new();
        for str in strs {
            let m = calc_key(&str);
            groups.entry(m).or_insert_with(|| Vec::new()).push(str);
        }
        groups.into_iter().map(|e| e.1).collect()
    }
}

fn main() {
    let strs = to_string_vec(vec![
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"]);
    println!("{:?}", Solution::group_anagrams(strs));
    let strs = to_string_vec(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
    println!("{:?}", Solution::group_anagrams(strs));
    let strs = to_string_vec(vec!["aaa", "aaa", "bb", "bb", "a", "a"]);
    println!("{:?}", Solution::group_anagrams(strs));
    let strs = to_string_vec(vec![""]);
    println!("{:?}", Solution::group_anagrams(strs));
}
