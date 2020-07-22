// https://leetcode.com/problems/sqrtx/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let x = x as i64;
    let mut left = 1;
    let mut right = x / 2;
    while left < right {
        let pivot = (left + right + 1) >> 1;
        let n = pivot * pivot;
        match n.cmp(&x) {
            Equal => return pivot as i32,
            Less => left = pivot,
            Greater => right = pivot - 1,
        }
    }
    left as i32
}
// math binary_search
#[test]
fn test1_69() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
}
