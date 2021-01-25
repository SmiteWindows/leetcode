// https://leetcode-cn.com/problems/number-of-digit-one/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn count_digit_one(n: i32) -> i32 {
    let mut m = 1i64;
    let mut res = 0i64;
    let n = n as i64;
    while m <= n {
        let d = 10 * m;
        res += n / d * m + m.min(0.max(n % d - m + 1));
        m *= 10;
    }
    res as i32
}
// math
#[test]
fn test1_233() {
    assert_eq!(count_digit_one(13), 6);
}
