// https://leetcode.com/problems/check-if-word-is-valid-after-substitutions/
pub fn is_valid(s: String) -> bool {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test2_1003() {
    assert_eq!(is_valid(String::from("aabcbc")), true);
    assert_eq!(is_valid(String::from("abcabcababcc")), true);
    assert_eq!(is_valid(String::from("abccba")), false);
    assert_eq!(is_valid(String::from("cababc")), false);
}
