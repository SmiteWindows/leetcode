// https://leetcode.com/problems/palindrome-pairs/
// Runtime: 92 ms
// Memory Usage: 3 MB
use std::collections::{HashMap, HashSet};
pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut id: HashMap<String, usize> = HashMap::new();
    let n = words.len();
    let mut res = HashSet::new();
    for (i, word) in words.iter().enumerate().take(n) {
        id.insert(word.to_string(), i);
    }
    for (i, word) in words.iter().enumerate().take(n) {
        let k = word.len();
        for mid in 0..=k {
            let left: String = word[0..mid].to_string();
            let right: String = word[mid..].to_string();
            if is_palindrome(&left) {
                let right_r: String = right.chars().rev().collect();
                if let Some(&j) = id.get(&right_r) {
                    if i != j {
                        res.insert(vec![j as i32, i as i32]);
                    }
                }
            }
            if is_palindrome(&right) {
                let left_r: String = left.chars().rev().collect();
                if let Some(&j) = id.get(&left_r) {
                    if i != j {
                        res.insert(vec![i as i32, j as i32]);
                    }
                }
            }
        }
    }
    res.into_iter().collect()
}

fn is_palindrome(s: &str) -> bool {
    !s.chars().zip(s.chars().rev()).any(|(a, b)| a != b)
}
// hash_table string trie
#[test]
fn test1_336() {
    assert_eq!(
        palindrome_pairs(vec![
            String::from("abcd"),
            String::from("dcba"),
            String::from("lls"),
            String::from("s"),
            String::from("sssll"),
        ]),
        vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]]
    );
    assert_eq!(
        palindrome_pairs(vec![
            String::from("bat"),
            String::from("tab"),
            String::from("cat"),
        ]),
        vec![vec![0, 1], vec![1, 0]]
    );
}
