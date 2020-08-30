// https://leetcode-cn.com/problems/add-and-search-word-data-structure-design/
// Runtime: 28 ms
// Memory Usage: 11.4 MB
use std::collections::HashMap;
#[derive(Default)]
struct WordDictionary {
    map: HashMap<char, Self>,
    is_word: bool,
}

impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            is_word: false,
        }
    }

    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        Self::add(self, &word);
    }

    fn add(wd: &mut WordDictionary, word: &str) {
        if let Some(c) = word.chars().next() {
            let next_wd = wd.map.entry(c).or_insert_with(WordDictionary::new);
            Self::add(next_wd, &word[1..]);
        } else {
            wd.is_word = true;
        }
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        Self::find(self, &word)
    }

    fn find(wd: &WordDictionary, word: &str) -> bool {
        match word.chars().next() {
            Some('.') => wd
                .map
                .values()
                .any(|next_wd| Self::find(next_wd, &word[1..])),
            Some(c) => wd
                .map
                .get(&c)
                .map_or(false, |next_wd| Self::find(next_wd, &word[1..])),
            None => wd.is_word,
        }
    }
}
/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
// design trie backtracking
#[test]
fn test3_211() {
    let mut wd = WordDictionary::new();
    wd.add_word("bad".to_string());
    wd.add_word("dad".to_string());
    wd.add_word("mad".to_string());
    assert_eq!(wd.search("pad".to_string()), false);
    assert_eq!(wd.search("bad".to_string()), true);
    assert_eq!(wd.search(".ad".to_string()), true);
    assert_eq!(wd.search("b..".to_string()), true);
}
