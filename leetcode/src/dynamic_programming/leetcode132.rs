// https://leetcode.com/problems/palindrome-partitioning-ii/
// Runtime: 72 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn min_cut(s: String) -> i32 {
    let n = s.len();
    let s: Vec<char> = s.chars().collect();
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
    dp(0, n, &mut memo, &s)
}

fn dp(start: usize, end: usize, memo: &mut HashMap<(usize, usize), i32>, s: &[char]) -> i32 {
    if let Some(&res) = memo.get(&(start, end)) {
        return res;
    }
    let res = if is_palindrome(start, end, s) {
        0
    } else {
        let mut res = std::i32::MAX;
        for i in start + 1..end {
            if is_palindrome(start, i, s) {
                res = res.min(1 + dp(i, end, memo, s));
            }
        }
        res
    };
    memo.insert((start, end), res);
    res
}

fn is_palindrome(start: usize, end: usize, s: &[char]) -> bool {
    !s[start..end]
        .iter()
        .zip(s[start..end].iter().rev())
        .any(|(a, b)| a != b)
}
// dynamic_programming
#[test]
fn test1_132() {
    assert_eq!(min_cut(String::from("aab")), 1);
}
