// https://leetcode.com/problems/longest-word-in-dictionary/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
use std::collections::HashSet;
pub fn longest_word(words: Vec<String>) -> String {
    let mut res = "".to_string();
    let mut words = words;
    let mut set = HashSet::new();
    words.sort();
    for word in words {
        if word.len() == 1 || set.contains(&word[..word.len() - 1]) {
            if word.len() > res.len() {
                res = word.to_string();
            }
            set.insert(word);
        }
    }
    res
}
// hash_table trie
#[test]
fn test2_720() {
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
