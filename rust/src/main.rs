mod helper;

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        assert!(begin_word.len() >= 1);
        assert_eq!(begin_word.len(), end_word.len());
        assert!(word_list.len() >= 1);

        use std::collections::HashMap;

        fn can_transfer(s: &String, t: &String) -> bool {
            assert_eq!(s.len(), t.len());
            let mut has_exactly_one_diff = false;
            for (x, y) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
                if x != y {
                    if has_exactly_one_diff {
                        return false;
                    }
                    has_exactly_one_diff = true;
                }
            }
            has_exactly_one_diff
        }

        fn build_transfer_map<'a>(
            word_list: &'a Vec<String>,
            begin_word: &'a String,
        ) -> HashMap<&'a String, Vec<(&'a String, usize)>> {
            let mut map = HashMap::new();
            for word in std::iter::once(begin_word)
                .chain(word_list.iter().filter(|&word| word != begin_word))
            {
                for i in 0..word_list.len() {
                    if can_transfer(word, &word_list[i]) {
                        map.entry(word).or_insert(vec![]).push((&word_list[i], i));
                    }
                }
            }
            map
        }

        fn build_distance_map<'a>(
            begin_word: &'a String,
            end_word: &'a String,
            transfer_map: &'a HashMap<&String, Vec<(&String, usize)>>,
        ) -> HashMap<&'a String, usize> {
            let mut level = 1;
            let mut found_ending_level = false;

            let mut map = HashMap::new();
            map.insert(begin_word, level);

            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back(begin_word);
            while !queue.is_empty() {
                let level_len = queue.len();
                for _ in 0..level_len {
                    let word = queue.pop_front().unwrap();
                    if let Some(next_words) = transfer_map.get(word) {
                        for &(next_word, _) in next_words {
                            if !map.contains_key(next_word) {
                                map.insert(next_word, level + 1);
                                if next_word == end_word {
                                    found_ending_level = true;
                                }

                                queue.push_back(next_word);
                            }
                        }
                    }
                }
                if found_ending_level {
                    break;
                }
                level += 1;
            }
            map
        }

        if !word_list.contains(&end_word) {
            return 0;
        }

        let transfer_map = build_transfer_map(&word_list, &begin_word);
        let distance_map = build_distance_map(&begin_word, &end_word, &transfer_map);
        if let Some(level) = distance_map.get(&end_word) {
            *level as i32
        } else {
            0
        }
    }
}

fn main() {
    use helper::util::to_string_vec;
    assert_eq!(
        5,
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            to_string_vec(vec!["hot", "dot", "dog", "lot", "log", "cog"])
        )
    );
    assert_eq!(
        0,
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            to_string_vec(vec!["hot", "dot", "dog", "lot", "log"])
        )
    );
}
