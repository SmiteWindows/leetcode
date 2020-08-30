// https://leetcode-cn.com/problems/number-of-segments-in-a-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
// ✔
pub fn count_segments(s: String) -> i32 {
    s.split_whitespace().count() as i32
}
// string
#[test]
fn test1_434() {
    assert_eq!(count_segments(String::from("Hello, my name is John")), 5);
}
