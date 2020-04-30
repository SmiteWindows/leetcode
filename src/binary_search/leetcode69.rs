// https://leetcode.com/problems/sqrtx/
use std::cmp::Ordering;
pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    let mut n: i64;
    let mut pivot: i32;
    let mut left = 2;
    let mut right = x / 2;
    while left <= right {
        pivot = left + (right - left) / 2;
        n = (pivot * pivot).into();
        let x = x as i64;
        match n.cmp(&x) {
            Ordering::Equal => return pivot,
            Ordering::Less => left = pivot + 1,
            Ordering::Greater => right = pivot - 1,
        }
    }
    right
}
// math binary_search
#[test]
fn test1_69() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
}
