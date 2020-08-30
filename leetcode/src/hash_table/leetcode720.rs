// https://leetcode-cn.com/problems/longest-word-in-dictionary/
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
