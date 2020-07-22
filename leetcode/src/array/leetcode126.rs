// https://leetcode.com/problems/word-ladder-ii/
// Runtime: 36 ms
// Memory Usage: 2.8 MB
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
pub fn find_ladders(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
) -> Vec<Vec<String>> {
    let mut dict = HashSet::new();
    for word in word_list {
        dict.insert(word);
    }
    if !dict.contains(&end_word) {
        return vec![];
    }
    let set1 = HashSet::from_iter(vec![begin_word.to_string()]);
    let set2 = HashSet::from_iter(vec![end_word.to_string()]);
    let mut map = HashMap::new();
    bfs(set1, set2, false, &mut map, &mut dict);
    let mut path = vec![begin_word.to_string()];
    let mut res = vec![];
    dfs(&begin_word, &mut path, &mut res, &map, &end_word);
    res
}

fn bfs(
    set1: HashSet<String>,
    set2: HashSet<String>,
    flipped: bool,
    map: &mut HashMap<String, HashSet<String>>,
    dict: &mut HashSet<String>,
) {
    if set1.is_empty() {
        return;
    }
    if set1.len() > set2.len() {
        bfs(set2, set1, !flipped, map, dict);
        return;
    }
    for s in set1.iter() {
        dict.remove(s);
    }
    for s in set2.iter() {
        dict.remove(s);
    }
    let mut next = HashSet::new();
    let mut done = false;
    for word in set1.iter() {
        for next_word in connected_words(&word.to_string()) {
            let key = if flipped {
                next_word.to_string()
            } else {
                word.to_string()
            };
            let value = if flipped {
                word.to_string()
            } else {
                next_word.to_string()
            };
            if set2.contains(&next_word) {
                done = true;
                map.entry(key).or_default().insert(value);
            } else if dict.contains(&next_word) {
                next.insert(next_word);
                map.entry(key).or_default().insert(value);
            }
        }
    }
    if !done {
        bfs(set2, next, !flipped, map, dict);
    }
}

fn dfs(
    start: &str,
    path: &mut Vec<String>,
    all: &mut Vec<Vec<String>>,
    map: &HashMap<String, HashSet<String>>,
    end: &str,
) {
    if start == end {
        all.push(path.to_vec());
    } else if let Some(nei) = map.get(start) {
        for next in nei.iter() {
            path.push(next.to_string());
            dfs(next, path, all, map, end);
            path.pop();
        }
    }
}

fn connected_words(word: &str) -> Vec<String> {
    let n = word.len();
    let mut res = vec![];
    for i in 0..n {
        let mut s = word.chars().collect::<Vec<char>>();
        for j in 0..26 {
            let c = (b'a' + j as u8) as char;
            s[i] = c;
            let new_word = s.iter().collect::<String>();
            res.push(new_word);
        }
    }
    res
}
// string backtracking array breadth_first_search
#[test]
fn test2_126() {
    let mut a = find_ladders(
        String::from("hit"),
        String::from("cog"),
        vec![
            String::from("hot"),
            String::from("dot"),
            String::from("dog"),
            String::from("lot"),
            String::from("log"),
            String::from("cog"),
        ],
    );
    a.sort();
    assert_eq!(
        a,
        vec![
            vec![
                String::from("hit"),
                String::from("hot"),
                String::from("dot"),
                String::from("dog"),
                String::from("cog")
            ],
            vec![
                String::from("hit"),
                String::from("hot"),
                String::from("lot"),
                String::from("log"),
                String::from("cog")
            ],
        ]
    );
    assert_eq!(
        find_ladders(
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
        vec![] as Vec<Vec<String>>
    );
}
