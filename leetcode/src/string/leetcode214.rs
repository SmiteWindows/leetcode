// https://leetcode-cn.com/problems/shortest-palindrome/
// Runtime: 0 ms
// Memory Usage: 3 MB
use std::iter::once;
pub fn shortest_palindrome(s: String) -> String {
    let n = s.len();
    let s_new = s
        .chars()
        .chain(once('#'))
        .chain(s.chars().rev())
        .collect::<Vec<_>>();
    let n_new = s_new.len();
    let mut f = vec![0; n_new];
    for i in 1..n_new {
        let mut t = f[i - 1];
        while t > 0 && s_new[i] != s_new[t] {
            t = f[t - 1];
        }
        if s_new[i] == s_new[t] {
            t += 1;
        }
        f[i] = t;
    }
    s.chars()
        .rev()
        .take(n - f.last().unwrap())
        .chain(s.chars())
        .collect()
}
// string
#[test]
fn test1_214() {
    assert_eq!(
        shortest_palindrome("aacecaaa".to_string()),
        "aaacecaaa".to_string()
    );
    assert_eq!(
        shortest_palindrome("abcd".to_string()),
        "dcbabcd".to_string()
    );
}
