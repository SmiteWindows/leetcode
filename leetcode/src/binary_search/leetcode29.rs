// https://leetcode.com/problems/divide-two-integers/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if let Some(res) = dividend.checked_div(divisor) {
        res
    } else {
        i32::MAX
    }
}
// math binary_search
#[test]
fn test1_29() {
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
}
