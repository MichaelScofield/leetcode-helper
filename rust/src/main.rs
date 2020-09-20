mod helper;

struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        assert!(text.len() > 0);
        let chars = text.chars().collect::<Vec<char>>();
        let len = chars.len();
        let mut spaces = 0;
        let mut words = 0;
        let mut letters = 0;
        let mut is_counting_word = false;
        for i in 0..len {
            if chars[i] == ' ' {
                spaces += 1;
                if is_counting_word {
                    is_counting_word = false;
                    words += 1;
                }
            } else {
                letters += 1;
                if !is_counting_word {
                    is_counting_word = true;
                }
            }
        }
        if is_counting_word {
            words += 1;
        }
        if words == 0 {
            return text;
        }
        let spaces_between_words = if words == 1 { 0 } else { spaces / (words - 1) };
        let mut s = Vec::with_capacity(len);
        let mut i = 0;
        while i < len {
            if chars[i] != ' ' {
                break;
            }
            i += 1;
        }
        while i < len && letters > 0 {
            if chars[i] != ' ' {
                s.push(chars[i]);
                i += 1;
                letters -= 1;
            } else {
                for _j in 0..spaces_between_words {
                    s.push(' ');
                }
                while i < len {
                    if chars[i] != ' ' {
                        break;
                    }
                    i += 1;
                }
            }
        }
        while s.len() < len {
            s.push(' ');
        }
        s.iter().collect()
    }
}

fn main() {
    assert_eq!("this   is   a   sentence".to_string(),
               Solution::reorder_spaces("  this   is  a sentence ".to_string()));
    assert_eq!("practice   makes   perfect ".to_string(),
               Solution::reorder_spaces(" practice   makes   perfect".to_string()));
    assert_eq!("hello   world".to_string(),
               Solution::reorder_spaces("hello   world".to_string()));
    assert_eq!("walks  udp  package  into  bar  a ".to_string(),
               Solution::reorder_spaces("  walks  udp package   into  bar a".to_string()));
    assert_eq!("a", Solution::reorder_spaces("a".to_string()));
    assert_eq!("  ", Solution::reorder_spaces("  ".to_string()));
}
