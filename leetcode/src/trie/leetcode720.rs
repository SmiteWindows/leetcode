// https://leetcode.com/problems/longest-word-in-dictionary/
// Runtime: 12 ms
// Memory Usage: 3 MB
use std::collections::BTreeMap;
pub fn longest_word(words: Vec<String>) -> String {
    let trie = Trie::from_words(words);
    let mut s = "".to_string();
    let mut max = "".to_string();
    trie.dfs(&mut s, &mut max);
    max
}

#[derive(Default)]
struct Trie {
    children: BTreeMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, s: String) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn from_words(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        trie
    }

    fn dfs(&self, s: &mut String, max: &mut String) {
        if self.end && s.len() > max.len() {
            *max = s.clone();
        }

        if s.is_empty() || self.end {
            for (&c, child) in self.children.iter() {
                s.push(c);
                child.dfs(s, max);
                s.pop();
            }
        }
    }
}
// hash_table trie
#[test]
fn test1_720() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        longest_word(vec_string!["w", "wo", "wor", "worl", "world"]),
        ("world").to_string()
    );
    assert_eq!(
        longest_word(vec_string![
            "a", "banana", "app", "appl", "ap", "apply", "apple"
        ]),
        ("apple").to_string()
    );
}
