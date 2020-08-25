// https://leetcode.com/problems/k-similar-strings/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
use std::collections::{HashSet, VecDeque};
pub fn k_similarity(a: String, b: String) -> i32 {
    let n = a.len();
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(a.clone());
    queue.push_back(a);
    let mut res = 0;
    while !queue.is_empty() {
        'outer: for _ in 0..queue.len() {
            let mut front = queue.pop_front().unwrap();
            let mut i = 0;
            while i < n && front[i] == b[i] {
                i += 1;
            }
            if i == n {
                return res;
            } else {
                for j in i + 1..n {
                    if front[j] == b[i] && front[i] == b[j] {
                        front.swap(i, j);
                        if visited.insert(front.clone()) {
                            queue.push_back(front.clone());
                        }
                        front.swap(i, j);
                        continue 'outer;
                    }
                }
                for j in i + 1..n {
                    if front[j] == b[i] {
                        front.swap(i, j);
                        if visited.insert(front.clone()) {
                            queue.push_back(front.clone());
                        }
                        front.swap(i, j);
                    }
                }
            }
        }
        res += 1;
    }
    0
}
// graph breadth_first_search
#[test]
fn test2_854() {
    assert_eq!(k_similarity(String::from("ab"), String::from("ba")), 1);
    assert_eq!(k_similarity(String::from("abc"), String::from("bca")), 2);
    assert_eq!(k_similarity(String::from("abac"), String::from("baca")), 2);
    assert_eq!(k_similarity(String::from("aabc"), String::from("abca")), 2);
}
