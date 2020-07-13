// https://leetcode.com/problems/word-break-ii/
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
            return vec![];
        }
    }
    let mut cur = vec![];
    let mut res = vec![];
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
        let mut words = vec![];
        for &(l, r) in cur.iter() {
            let mut word = "".to_string();
            for i in l..=r {
                word.push(s[i] as char);
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
    assert_eq!(
        word_break(
            String::from("catsanddog"),
            vec![
                String::from("cat"),
                String::from("cats"),
                String::from("and"),
                String::from("sand"),
                String::from("dog")
            ]
        ),
        vec![String::from("cat sand dog"), String::from("cats and dog")]
    );
    assert_eq!(
        word_break(
            String::from("pineapplepenapple"),
            vec![
                String::from("apple"),
                String::from("pen"),
                String::from("applepen"),
                String::from("pine"),
                String::from("pineapple")
            ]
        ),
        vec![
            String::from("pine apple pen apple"),
            String::from("pine applepen apple"),
            String::from("pineapple pen apple"),
        ]
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
        vec![] as Vec<String>
    );
}
