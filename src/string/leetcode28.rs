// https://leetcode.com/problems/implement-strstr/
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
    assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
    assert_eq!(str_str(String::from("aaaaa"), String::from("bba")), -1);
}
