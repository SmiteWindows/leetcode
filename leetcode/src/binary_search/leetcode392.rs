// https://leetcode.com/problems/is-subsequence/
pub fn is_subsequence(s: String, t: String) -> bool {
    todo!()
}
// binary_search dynamic_programming greedy
#[test]
#[ignore]
fn test1_392() {
    assert_eq!(
        is_subsequence(String::from("abc"), String::from("ahbgdc")),
        true
    );
    assert_eq!(
        is_subsequence(String::from("axc"), String::from("ahbgdc")),
        false
    );
}
