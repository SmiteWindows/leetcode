// https://leetcode-cn.com/problems/number-of-segments-in-a-string/
pub fn count_segments(s: String) -> i32 {
    s.split_whitespace().count() as i32
}
// Runtime: 0 ms
// Memory Usage: 2 MB
// ✔
// string
#[test]
fn test1_434() {
    assert_eq!(count_segments("Hello, my name is John".to_string()), 5);
}
