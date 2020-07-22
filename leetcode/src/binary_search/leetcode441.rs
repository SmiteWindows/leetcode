// https://leetcode.com/problems/arranging-coins/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn arrange_coins(n: i32) -> i32 {
    let n = n as i64;
    let mut l = 1;
    let mut r = n;
    while l <= r {
        let k = l + (r - l) / 2;
        let target = (k + 1) * k / 2;
        match target.cmp(&n) {
            Equal => return k as i32,
            Less => l = k + 1,
            Greater => r = k - 1,
        }
    }
    r as i32
}
// math binary_search
#[test]
fn test2_441() {
    assert_eq!(arrange_coins(5), 2);
    assert_eq!(arrange_coins(8), 3);
}
