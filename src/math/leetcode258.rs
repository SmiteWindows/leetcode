// https://leetcode.com/problems/add-digits/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn add_digits(num: i32) -> i32 {
    (num - 1) % 9 + 1
}
// math
#[test]
fn test1_258() {
    assert_eq!(add_digits(38), 2);
}
