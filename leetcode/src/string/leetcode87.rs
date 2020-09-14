// https://leetcode-cn.com/problems/scramble-string/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn is_scramble(s1: String, s2: String) -> bool {
    let n = s1.len();
    let mut memo = HashMap::new();
    dp(0, n, 0, n, &mut memo, &s1, &s2)
}

fn dp(
    l_start: usize,
    l_end: usize,
    r_start: usize,
    r_end: usize,
    memo: &mut HashMap<(usize, usize, usize, usize), bool>,
    s1: &str,
    s2: &str,
) -> bool {
    if let Some(&res) = memo.get(&(l_start, l_end, r_start, r_end)) {
        return res;
    }
    let res = if s1[l_start..l_end] == s2[r_start..r_end] {
        true
    } else {
        let mut res = false;
        let n = l_end - l_start;
        for i in 1..n {
            if dp(l_start, l_start + i, r_start, r_start + i, memo, s1, s2)
                && dp(l_start + i, l_end, r_start + i, r_end, memo, s1, s2)
                || dp(l_start, l_start + i, r_end - i, r_end, memo, s1, s2)
                    && dp(l_start + i, l_end, r_start, r_end - i, memo, s1, s2)
            {
                res = true;
                break;
            }
        }
        res
    };
    memo.insert((l_start, l_end, r_start, r_end), res);
    res
}
// dynamic_programming string
#[test]
fn test2_87() {
    assert_eq!(is_scramble("great".to_string(), "rgeat".to_string()), true);
    assert_eq!(is_scramble("abcde".to_string(), "caebd".to_string()), false);
}
