// https://leetcode-cn.com/problems/word-break-ii/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::Hasher,
};
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let n = s.len();
    let mut dict = HashSet::new();
    let mut alphabet = vec![false; 256];
    for word in word_dict {
        let mut hasher = DefaultHasher::new();
        for b in word.bytes() {
            alphabet[b as usize] = true;
            hasher.write_u8(b);
        }
        dict.insert(hasher.finish());
    }
    let s = s.bytes().collect::<Vec<u8>>();
    for i in 0..n {
        if !alphabet[s[i] as usize] {
            return Vec::new();
        }
    }
    let mut cur = Vec::new();
    let mut res = Vec::new();
    dfs(0, &mut cur, &mut res, &dict, &s, n);
    res
}

fn dfs(
    start: usize,
    cur: &mut Vec<(usize, usize)>,
    all: &mut Vec<String>,
    dict: &HashSet<u64>,
    s: &[u8],
    n: usize,
) {
    if start == n {
        let mut words = Vec::new();
        for &(l, r) in cur.iter() {
            let mut word = "".to_string();
            for &si in s.iter().take(r + 1).skip(l) {
                word.push(si as char);
            }
            words.push(word);
        }
        all.push(words.join(" "));
    }
    let mut hasher = DefaultHasher::new();
    for i in start..n {
        hasher.write_u8(s[i]);
        if dict.contains(&hasher.finish()) {
            cur.push((start, i));
            dfs(i + 1, cur, all, dict, s, n);
            cur.pop();
        }
    }
}
// backtracking dynamic_programming
#[test]
fn test1_140() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        word_break(
            "catsanddog".to_string(),
            vec_string!["cat", "cats", "and", "sand", "dog"]
        ),
        vec_string!["cat sand dog", "cats and dog"]
    );
    assert_eq!(
        word_break(
            "pineapplepenapple".to_string(),
            vec_string!["apple", "pen", "applepen", "pine", "pineapple"]
        ),
        vec_string![
            "pine apple pen apple",
            "pine applepen apple",
            "pineapple pen apple"
        ]
    );
    assert_eq!(
        word_break(
            "catsandog".to_string(),
            vec_string!["cats", "dog", "sand", "and", "cat"]
        ),
        vec_string![]
    );
}
