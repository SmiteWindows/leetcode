// https://leetcode-cn.com/problems/word-pattern/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn word_pattern(pattern: String, str: String) -> bool {
    let chars = pattern.chars().collect::<Vec<_>>();
    let strings = str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    if chars.len() != strings.len() {
        return false;
    }
    let mut hm1 = HashMap::new();
    let mut hm2 = HashMap::new();
    for (i, &ch) in chars.iter().enumerate() {
        let string = strings[i].clone();
        if let Some(s) = hm1.get(&ch) {
            if *s != string {
                return false;
            }
        } else {
            hm1.insert(ch, string.clone());
        }
        if let Some(c) = hm2.get(&string) {
            if *c != ch {
                return false;
            }
        } else {
            hm2.insert(string.clone(), ch);
        }
    }
    true
}
// hash_table
#[test]
fn test1_290() {
    assert_eq!(
        word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
    assert_eq!(
        word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
        false
    );
    assert_eq!(
        word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
        false
    );
    assert_eq!(
        word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
        false
    );
}
