// https://leetcode-cn.com/problems/replace-words/
// Runtime: 4 ms
// Memory Usage: 11.2 MB
use std::collections::HashMap;
pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
    let mut trie = Trie::new();
    for word in dict {
        trie.insert(word);
    }
    sentence
        .split_whitespace()
        .map(|s| trie.map(s))
        .collect::<Vec<&str>>()
        .join(" ")
}

#[derive(Default)]
struct Trie {
    children: HashMap<char, Self>,
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

    fn map<'a>(&self, s: &'a str) -> &'a str {
        let mut link = self;
        for (size, c) in s.char_indices() {
            if link.end {
                return &s[0..size];
            }
            if let Some(next) = link.children.get(&c) {
                link = next;
            } else {
                break;
            }
        }
        s
    }
}
// hash_table trie
#[test]
fn test1_648() {
    assert_eq!(
        replace_words(
            vec![
                "cat"),
                "bat"),
                "rat"),
            ],
            "the cattle was rattled by the battery")
        ),
        "the cat was rat by the bat"),
    );
}
