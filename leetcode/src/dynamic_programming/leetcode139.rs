// https://leetcode-cn.com/problems/word-break/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{collections::HashSet, iter::FromIterator};
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut a = vec![false; n + 1];
    let hs: HashSet<String> = HashSet::from_iter(word_dict);
    a[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if a[j] && hs.contains(&s[j..i]) {
                a[i] = true;
                break;
            }
        }
    }
    a[n]
}
// dynamic_programming
#[test]
fn test1_139() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        word_break("leetcode".to_string(), vec_string!["leet", "code"]),
        true
    );
    assert_eq!(
        word_break("applepenapple".to_string(), vec_string!["apple", "pen"]),
        true
    );
    assert_eq!(
        word_break(
            "catsandog".to_string(),
            vec_string!["cats", "dog", "sand", "and", "cat"]
        ),
        false
    );
}
