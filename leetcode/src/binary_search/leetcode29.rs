// https://leetcode-cn.com/problems/divide-two-integers/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::convert::identity;
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    dividend.checked_div(divisor).map_or(i32::MAX, identity)
}
// math binary_search
#[test]
fn test1_29() {
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
}
