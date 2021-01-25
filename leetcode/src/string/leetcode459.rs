// https://leetcode-cn.com/problems/repeated-substring-pattern/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::convert::identity;
pub fn repeated_substring_pattern(s: String) -> bool {
    (1..s.len())
        .filter(|&step| s.len() % step == 0)
        .map(|step| {
            (step..s.len())
                .step_by(step)
                .all(|front| s[front..step + front] == s[..step])
        })
        .any(identity)
}
// string
#[test]
fn test1_459() {
    assert_eq!(repeated_substring_pattern("abab".to_string()), true);
    assert_eq!(repeated_substring_pattern("aba".to_string()), false);
    assert_eq!(repeated_substring_pattern("abcabcabcabc".to_string()), true);
}
