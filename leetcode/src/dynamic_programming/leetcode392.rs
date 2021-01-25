// https://leetcode-cn.com/problems/is-subsequence/
pub fn is_subsequence(s: String, t: String) -> bool {
    todo!()
}
// binary_search dynamic_programming greedy
#[test]
#[ignore]
fn test2_392() {
    assert_eq!(
        is_subsequence("abc".to_string(), "ahbgdc".to_string()),
        true
    );
    assert_eq!(
        is_subsequence("axc".to_string(), "ahbgdc".to_string()),
        false
    );
}
