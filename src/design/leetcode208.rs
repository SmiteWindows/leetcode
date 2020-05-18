// https://leetcode.com/problems/implement-trie-prefix-tree/
// Runtime: 20 ms
// Memory Usage: 11.7 MB
use std::collections::HashMap;
#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end: false,
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut trie = self;
        for c in word.chars() {
            trie = trie.children.entry(c).or_default();
        }
        trie.end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut trie = self;
        for c in word.chars() {
            if let Some(child) = trie.children.get(&c) {
                trie = child;
            } else {
                return false;
            }
        }
        trie.end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut trie = self;
        for c in prefix.chars() {
            if let Some(child) = trie.children.get(&c) {
                trie = child;
            } else {
                return false;
            }
        }
        true
    }
}
/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// design trie
#[test]
fn test2_208() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
