// https://leetcode-cn.com/problems/is-subsequence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    let sn = s.len();
    let tm = t.len();
    while i < sn && j < tm {
        if s[i..=i] == t[j..=j] {
            i += 1;
        }
        j += 1;
    }
    i == sn
}
// binary_search dynamic_programming greedy
#[test]
fn test3_392() {
    assert_eq!(
        is_subsequence("abc".to_string(), "ahbgdc".to_string()),
        true
    );
    assert_eq!(
        is_subsequence("axc".to_string(), "ahbgdc".to_string()),
        false
    );
}
