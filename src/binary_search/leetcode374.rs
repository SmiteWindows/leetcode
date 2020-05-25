// https://leetcode.com/problems/guess-number-higher-or-lower/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
#[allow(non_snake_case)]
unsafe fn guessNumber(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;
    while l <= r {
        let m = l + (r - l) / 2;
        match guess(m) {
            0 => return m,
            -1 => r = m - 1,
            1 => l = m + 1,
            _ => {}
        }
    }
    1
}
/**
* Forward declaration of guess API.
* @param  num   your guess
* @return 	     -1 if num is lower than the guess number
*			      1 if num is higher than the guess number
*               otherwise return 0
* unsafe fn guess(num: i32) -> i32 {}
*/
unsafe fn guess(num: i32) -> i32 {
    use std::cmp::Ordering::{Equal, Greater, Less};
    match X.cmp(&num) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}
// pick = 6
const X: i32 = 6;
// binary_search
#[test]
fn test1_374() {
    unsafe {
        assert_eq!(guessNumber(10), 6);
    }
}
