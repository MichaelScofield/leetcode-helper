mod helper;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word_ending: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        assert!(word.len() >= 1);

        let mut trie = self;
        for ch in word.chars() {
            trie = trie.children[(ch as usize - 'a' as usize)]
                .get_or_insert_with(|| Box::new(Trie::new()));
        }
        trie.is_word_ending = true
    }

    fn search(&self, word: String) -> bool {
        assert!(word.len() >= 1);

        if let Some(trie) = self.search0(word) {
            trie.is_word_ending
        } else {
            false
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        assert!(prefix.len() >= 1);

        self.search0(prefix).is_some()
    }

    fn search0(&self, word: String) -> Option<&Trie> {
        assert!(word.len() >= 1);

        let mut trie = self;
        for ch in word.chars() {
            if let Some(x) = trie.children[(ch as usize - 'a' as usize)].as_ref() {
                trie = x;
            } else {
                return None;
            }
        }
        Some(trie)
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("app".to_string());
    trie.insert("apple".to_string());
    trie.insert("beer".to_string());
    trie.insert("add".to_string());
    trie.insert("jam".to_string());
    trie.insert("rental".to_string());
    assert!(!trie.search("apps".to_string()));
    assert!(trie.search("app".to_string()));
    assert!(!trie.search("ad".to_string()));
    assert!(!trie.search("applepie".to_string()));
    assert!(!trie.search("rest".to_string()));
    assert!(!trie.search("jan".to_string()));
    assert!(!trie.search("rent".to_string()));
    assert!(trie.search("beer".to_string()));
    assert!(trie.search("jam".to_string()));
    assert!(!trie.starts_with("apps".to_string()));
    assert!(trie.starts_with("app".to_string()));
    assert!(trie.starts_with("ad".to_string()));
    assert!(!trie.starts_with("applepie".to_string()));
    assert!(!trie.starts_with("rest".to_string()));
    assert!(!trie.starts_with("jan".to_string()));
    assert!(trie.starts_with("rent".to_string()));
    assert!(trie.starts_with("beer".to_string()));
    assert!(trie.starts_with("jam".to_string()));
}
