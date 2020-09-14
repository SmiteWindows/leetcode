// https://leetcode-cn.com/problems/top-k-frequent-words/
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
fn test1_692() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        top_k_frequent(
            vec_string!["i", "love", "leetcode", "i", "love", "coding"],
            2
        ),
        vec_string!["i", "love"]
    );
    assert_eq!(
        top_k_frequent(
            vec_string!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"],
            4
        ),
        vec_string!["the", "is", "sunny", "day"]
    );
}
