// https://leetcode-cn.com/problems/word-ladder/
// Runtime: 12 ms
// Memory Usage: 2.6 MB
use std::{collections::HashSet, iter::FromIterator, mem::swap};
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let n = begin_word.len();
    let mut unused_set: HashSet<Vec<u8>> =
        HashSet::from_iter(word_list.into_iter().map(|s| s.as_bytes().to_vec()));
    let begin_word = begin_word.as_bytes().to_vec();
    let end_word = end_word.as_bytes().to_vec();
    if !unused_set.contains(&end_word) {
        return 0;
    }
    let begin_list = vec![begin_word];
    let end_list = vec![end_word];
    let mut begin_set: HashSet<Vec<u8>> = HashSet::from_iter(begin_list);
    let mut end_set: HashSet<Vec<u8>> = HashSet::from_iter(end_list);
    let mut left_set = &mut begin_set;
    let mut right_set = &mut end_set;
    let mut ladder = 1;
    while !left_set.is_empty() {
        ladder += 1;
        let mut next_set = HashSet::new();
        for s in left_set.iter() {
            let mut v = s.to_vec();
            for i in 0..n {
                let c = v[i];
                for j in 0..26 {
                    v[i] = b'a' + j;
                    if right_set.contains(&v) {
                        return ladder;
                    }
                    if unused_set.contains(&v) {
                        unused_set.remove(&v);
                        next_set.insert(v.to_vec());
                    }
                }
                v[i] = c;
            }
        }
        *left_set = next_set;
        if left_set.len() > right_set.len() {
            swap(&mut left_set, &mut right_set)
        }
    }
    0
}
// breadth_first_search
#[test]
fn test1_127() {
    assert_eq!(
        ladder_length(
            String::from("hit"),
            String::from("cog"),
            vec![
                String::from("hot"),
                String::from("dot"),
                String::from("dog"),
                String::from("lot"),
                String::from("log"),
                String::from("cog")
            ]
        ),
        5
    );
    assert_eq!(
        ladder_length(
            String::from("hit"),
            String::from("cog"),
            vec![
                String::from("hot"),
                String::from("dot"),
                String::from("dog"),
                String::from("lot"),
                String::from("log")
            ]
        ),
        0
    );
}
