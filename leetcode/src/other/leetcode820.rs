// https://leetcode.com/problems/short-encoding-of-words/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
use std::collections::HashSet;
pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
    let mut hashset = words.iter().cloned().collect::<HashSet<String>>();
    for word in words.iter() {
        let len = word.len();
        for i in 1..len {
            hashset.remove(&word[i..len]);
        }
    }
    let mut count = 0;
    for word in hashset {
        count += (word.len() as i32) + 1;
    }
    count
}
#[test]
fn test820() {
    assert_eq!(
        minimum_length_encoding(vec![
            "time".to_string(),
            "me".to_string(),
            "bell".to_string()
        ]),
        10
    );
}
