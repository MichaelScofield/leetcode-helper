mod helper;

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut chars = s.trim().chars().collect::<Vec<char>>();
        let len = chars.len();
        if len == 0 {
            return "".to_string();
        }
        chars.reverse();
        let mut reverse: Vec<char> = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = 1;
        while j < len {
            if chars[i] == ' ' {
                if chars[j] != ' ' {
                    i = j;
                }
            } else {
                if chars[j] == ' ' {
                    let word = &mut chars[i..j].to_vec();
                    word.reverse();
                    reverse.append(word);
                    reverse.push(' ');
                    i = j;
                }
            }
            j += 1;
        }
        if chars[i] != ' ' {
            let word = &mut chars[i..len].to_vec();
            word.reverse();
            reverse.append(word);
        }
        reverse.iter().collect()
    }
}

fn main() {
    assert_eq!("blue is sky the".to_string(), Solution::reverse_words("the sky is blue".to_string()));
    assert_eq!("world! hello".to_string(), Solution::reverse_words("  hello world!  ".to_string()));
    assert_eq!("example good a".to_string(), Solution::reverse_words("a good   example".to_string()));
    assert_eq!("example".to_string(), Solution::reverse_words("example".to_string()));
    assert_eq!("".to_string(), Solution::reverse_words("  ".to_string()));
}
