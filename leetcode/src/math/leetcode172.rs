// https://leetcode-cn.com/problems/factorial-trailing-zeroes/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn trailing_zeroes(mut n: i32) -> i32 {
    let mut res = 0;
    while n > 0 {
        res += n / 5;
        n /= 5;
    }
    res
}
// math
#[test]
fn test1_172() {
    assert_eq!(trailing_zeroes(3), 0);
    assert_eq!(trailing_zeroes(5), 1);
}
