// https://leetcode.com/problems/repeated-substring-pattern/
pub fn repeated_substring_pattern(s: String) -> bool {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_459() {
    assert_eq!(repeated_substring_pattern(String::from("abab")), true);
    assert_eq!(repeated_substring_pattern(String::from("aba")), false);
    assert_eq!(
        repeated_substring_pattern(String::from("abcabcabcabc")),
        true
    );
}
