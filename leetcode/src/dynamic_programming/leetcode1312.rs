// https://leetcode-cn.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// Runtime: 200 ms
// Memory Usage: 8.3 MB
use std::collections::HashMap;
pub fn min_insertions(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<char>>();
    let mut memo = HashMap::new();
    dp(0, n, &mut memo, &s)
}

fn dp(start: usize, end: usize, memo: &mut HashMap<(usize, usize), i32>, s: &[char]) -> i32 {
    let n = end - start;
    if n < 2 {
        return 0;
    }
    if let Some(&res) = memo.get(&(start, end)) {
        return res;
    }
    let res = if s[start] == s[end - 1] {
        dp(start + 1, end - 1, memo, s)
    } else {
        1 + dp(start, end - 1, memo, s).min(dp(start + 1, end, memo, s))
    };
    memo.insert((start, end), res);
    res
}
// dynamic_programming
#[test]
fn test1_1312() {
    assert_eq!(min_insertions("zzazz".to_string()), 0);
    assert_eq!(min_insertions("mbadm".to_string()), 2);
    assert_eq!(min_insertions("leetcode".to_string()), 5);
    assert_eq!(min_insertions("g".to_string()), 0);
    assert_eq!(min_insertions("no".to_string()), 1);
}
