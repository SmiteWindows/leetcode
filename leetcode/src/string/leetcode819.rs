// https://leetcode-cn.com/problems/most-common-word/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::{HashMap, HashSet};
pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let p = paragraph
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>();
    let mut hm: HashMap<String, usize> = HashMap::new();
    for word in p.split_whitespace() {
        *hm.entry(word.to_string()).or_default() += 1;
    }
    let mut max = 0;
    let mut res = "".to_string();
    let banned = banned.into_iter().collect::<HashSet<_>>();
    for word in p.split_whitespace() {
        if !banned.contains(word) {
            if let Some(&count) = hm.get(word) {
                if count > max {
                    max = count;
                    res = word.to_string();
                }
            }
        }
    }
    res
}
// string
#[test]
fn test1_819() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec_string!["hit"]
        ),
        "ball".to_string()
    );
}
