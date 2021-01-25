// https://leetcode-cn.com/problems/implement-strstr/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1,
    }
}
// string two_pointers
#[test]
fn test2_28() {
    assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
}
