// https://leetcode.com/problems/rotate-string/
pub fn rotate_string(a: String, b: String) -> bool {
    todo!()
}
#[test]
#[ignore]
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
