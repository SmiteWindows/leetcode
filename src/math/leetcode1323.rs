// https://leetcode.com/problems/maximum-69-number/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn maximum69_number(num: i32) -> i32 {
    num.to_string().replacen("6", "9", 1).parse().unwrap()
}
// math
#[test]
fn test1_1323() {
    assert_eq!(maximum69_number(9669), 9969);
    assert_eq!(maximum69_number(9996), 9999);
    assert_eq!(maximum69_number(9999), 9999);
}
