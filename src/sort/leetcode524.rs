// // https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/
// Runtime: 4 ms
// Memory Usage: 2.8 MB
use std::{cmp::Reverse, collections::BinaryHeap};
pub fn find_longest_word(s: String, d: Vec<String>) -> String {
    let s = s.chars().collect::<Vec<_>>();
    let mut queue = d
        .into_iter()
        .map(|s| (s.len(), Reverse(s)))
        .collect::<BinaryHeap<(usize, Reverse<String>)>>();
    while let Some((_, r)) = queue.pop() {
        let mut it = r.0.chars().peekable();
        for i in 0..s.len() {
            if let Some(&c) = it.peek() {
                if c == s[i] {
                    it.next();
                }
            } else {
                break;
            }
        }
        if it.next().is_none() {
            return r.0;
        }
    }
    "".to_string()
}
// sort two_pointers
#[test]
fn test2_524() {
    assert_eq!(
        find_longest_word(
            String::from("abpcplea"),
            vec![
                String::from("ale"),
                String::from("apple"),
                String::from("monkey"),
                String::from("plea")
            ]
        ),
        String::from("apple")
    );
    assert_eq!(
        find_longest_word(
            String::from("abpcplea"),
            vec![String::from("a"), String::from("b"), String::from("c")]
        ),
        String::from("a")
    );
}
