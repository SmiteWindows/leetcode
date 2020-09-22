// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
// Runtime: 48 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn max_unique_split(s: String) -> i32 {
    let n = s.len();
    let mut visited: HashSet<String> = HashSet::new();
    let mut res = 0;
    dfs(0, 0, &mut visited, &mut res, &s, n);
    res as i32
}

fn dfs(
    start: usize,
    cur: usize,
    visited: &mut HashSet<String>,
    max: &mut usize,
    s: &str,
    n: usize,
) {
    if start == n {
        *max = (*max).max(cur);
    } else {
        for i in start..n {
            if !visited.contains(&s[start..=i]) {
                visited.insert(s[start..=i].to_string());
                dfs(i + 1, cur + 1, visited, max, s, n);
                visited.remove(&s[start..=i]);
            }
        }
    }
}
// backtracking
#[test]
fn test1_593() {
    assert_eq!(max_unique_split("ababccc".to_string()), 5);
    assert_eq!(max_unique_split("aba".to_string()), 2);
    assert_eq!(max_unique_split("aa".to_string()), 1);
}
