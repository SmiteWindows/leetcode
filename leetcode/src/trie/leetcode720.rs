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
    assert_eq!(
        longest_word(vec![
            String::from("w"),
            String::from("wo"),
            String::from("wor"),
            String::from("worl"),
            String::from("world"),
        ]),
        String::from("world")
    );
    assert_eq!(
        longest_word(vec![
            String::from("a"),
            String::from("banana"),
            String::from("app"),
            String::from("appl"),
            String::from("ap"),
            String::from("apply"),
            String::from("apple"),
        ]),
        String::from("apple"),
    );
}
