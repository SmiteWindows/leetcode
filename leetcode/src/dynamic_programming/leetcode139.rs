// https://leetcode.com/problems/word-break/
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
    assert_eq!(
        word_break(
            String::from("leetcode"),
            vec![String::from("leet"), String::from("code")]
        ),
        true
    );
    assert_eq!(
        word_break(
            String::from("applepenapple"),
            vec![String::from("apple"), String::from("pen")]
        ),
        true
    );
    assert_eq!(
        word_break(
            String::from("catsandog"),
            vec![
                String::from("cats"),
                String::from("dog"),
                String::from("sand"),
                String::from("and"),
                String::from("cat")
            ]
        ),
        false
    );
}
