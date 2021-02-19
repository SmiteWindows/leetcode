// https://leetcode-cn.com/problems/longest-happy-prefix/
pub fn longest_prefix(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1392() {
    assert_eq!(longest_prefix("level".to_string()), "l".to_string());
    assert_eq!(longest_prefix("ababab".to_string()), "abab".to_string());
    assert_eq!(
        longest_prefix("leetcodeleet".to_string()),
        "leet".to_string()
    );
    assert_eq!(longest_prefix("a".to_string()), "".to_string());
}
