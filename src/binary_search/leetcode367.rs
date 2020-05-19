// https://leetcode.com/problems/valid-perfect-square/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering;
pub fn is_perfect_square(num: i32) -> bool {
    if num < 2 {
        return true;
    }
    let num = num as i64;
    let mut left = 2_i64;
    let mut right = num / 2;
    while left <= right {
        let x = left + (right - left) / 2;
        let n = x * x;
        match n.cmp(&num) {
            Ordering::Equal => return true,
            Ordering::Less => left = x + 1,
            Ordering::Greater => right = x - 1,
        }
    }
    false
}
// math binary_search
#[test]
fn test2_367() {
    assert_eq!(is_perfect_square(16), true);
    assert_eq!(is_perfect_square(14), false);
}
