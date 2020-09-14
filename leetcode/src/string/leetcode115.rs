// https://leetcode-cn.com/problems/distinct-subsequences/
// Runtime: 40 ms
// Memory Usage: 22.7 MB
#![allow(clippy::many_single_char_names)]
use std::collections::HashMap;
pub fn num_distinct(s: String, t: String) -> i32 {
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let n = s.len();
    let m = t.len();
    dp(0, 0, &mut memo, &s, &t, n, m)
}

fn dp(
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), i32>,
    s: &[char],
    t: &[char],
    n: usize,
    m: usize,
) -> i32 {
    if let Some(&res) = memo.get(&(i, j)) {
        return res;
    }
    let res = if j == m {
        1
    } else if i == n {
        0
    } else if s[i] == t[j] {
        dp(i + 1, j + 1, memo, s, t, n, m) + dp(i + 1, j, memo, s, t, n, m)
    } else {
        dp(i + 1, j, memo, s, t, n, m)
    };
    memo.insert((i, j), res);
    res
}
// dynamic_programming string
#[test]
fn test2_115() {
    assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
}
