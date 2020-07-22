// https://leetcode.com/problems/repeated-substring-pattern/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn repeated_substring_pattern(s: String) -> bool {
    let mut t = "".to_string();
    let n = s.len();
    if n < 2 {
        return false;
    }
    t += &s;
    t += &s;
    t[1..2 * n - 1].find(&s).is_some()
}
// string
#[test]
fn test1_459() {
    assert_eq!(repeated_substring_pattern(String::from("abab")), true);
    assert_eq!(repeated_substring_pattern(String::from("aba")), false);
    assert_eq!(
        repeated_substring_pattern(String::from("abcabcabcabc")),
        true
    );
}
