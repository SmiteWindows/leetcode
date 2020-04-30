// https://leetcode.com/problems/number-of-digit-one/
use std::cmp::{max, min};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn count_digit_one(n: i32) -> i32 {
    let mut res = 0;
    let mut i = 1_i64;
    let n = n as i64;
    while i <= n {
        let d = i * 10 as i64;
        res += (n / d) * i + min(max(n % d - i + 1, 0), i);
        i *= 10;
    }
    res as i32
}
// math
#[test]
fn test1_233() {
    assert_eq!(count_digit_one(13), 6);
}
