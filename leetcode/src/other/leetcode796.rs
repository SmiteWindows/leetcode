// https://leetcode.com/problems/rotate-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn rotate_string(a: String, b: String) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut c = "".to_string();
    c += &a;
    c += &a;
    c.contains(&b)
}
#[test]
fn test796() {
    assert_eq!(
        rotate_string("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        rotate_string("abcde".to_string(), "abced".to_string()),
        false
    );
}
