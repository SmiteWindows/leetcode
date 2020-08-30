// https://leetcode-cn.com/problems/concatenated-words/
// Runtime: 76 ms
// Memory Usage: 3.4 MB
use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::Hasher,
};
pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    let mut hs = HashSet::new();
    for w in &words {
        let mut hasher = DefaultHasher::new();
        for b in w.bytes() {
            hasher.write_u8(b);
        }
        hs.insert(hasher.finish());
    }
    let mut res = vec![];
    for w in words {
        let s = w.bytes().collect::<Vec<u8>>();
        let n = s.len();
        if dfs(0, 0, &hs, &s, n) {
            res.push(w);
        }
    }
    res
}

fn dfs(start: usize, k: usize, hs: &HashSet<u64>, s: &[u8], n: usize) -> bool {
    if start == n {
        k >= 2
    } else {
        let mut hasher = DefaultHasher::new();
        for i in start..n {
            hasher.write_u8(s[i]);
            if hs.contains(&hasher.finish()) && dfs(i + 1, k + 1, hs, s, n) {
                return true;
            }
        }
        false
    }
}
// dynamic_programming depth_first_search trie
#[test]
fn test3_472() {
    assert_eq!(
        find_all_concatenated_words_in_a_dict(vec![
            String::from("cat"),
            String::from("cats"),
            String::from("catsdogcats"),
            String::from("dog"),
            String::from("dogcatsdog"),
            String::from("hippopotamuses"),
            String::from("rat"),
            String::from("ratcatdogcat"),
        ]),
        vec![
            String::from("catsdogcats"),
            String::from("dogcatsdog"),
            String::from("ratcatdogcat"),
        ],
    );
}
