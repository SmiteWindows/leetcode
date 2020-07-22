// https://leetcode.com/problems/top-k-frequent-words/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cmp::Ordering, collections::HashMap};
pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut hm: HashMap<&str, usize> = HashMap::new();
    let mut v = vec![];
    for w in words.iter() {
        *hm.entry(&w).or_default() += 1;
    }
    for (word, freq) in hm {
        v.push(Pair { word, freq });
    }
    v.sort_unstable_by(|a, b| match b.freq.cmp(&a.freq) {
        Ordering::Equal => a.word.cmp(b.word),
        e => e,
    });
    v.iter()
        .take(k as usize)
        .map(|x| x.word.to_string())
        .collect()
}

struct Pair<'a> {
    word: &'a str,
    freq: usize,
}
// hash_table trie heap
#[test]
fn test3_692() {
    assert_eq!(
        top_k_frequent(
            vec![
                String::from("i"),
                String::from("love"),
                String::from("leetcode"),
                String::from("i"),
                String::from("love"),
                String::from("coding"),
            ],
            2
        ),
        vec![String::from("i"), String::from("love")]
    );
    assert_eq!(
        top_k_frequent(
            vec![
                String::from("the"),
                String::from("day"),
                String::from("is"),
                String::from("sunny"),
                String::from("the"),
                String::from("the"),
                String::from("the"),
                String::from("sunny"),
                String::from("is"),
                String::from("is"),
            ],
            4
        ),
        vec![
            String::from("the"),
            String::from("is"),
            String::from("sunny"),
            String::from("day")
        ]
    );
}
