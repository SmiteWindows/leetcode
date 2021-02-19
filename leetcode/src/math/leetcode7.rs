// https://leetcode-cn.com/problems/reverse-integer/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn reverse(mut x: i32) -> i32 {
    let mut rev = 0;
    while x != 0 {
        let pop = x % 10;
        x /= 10;
        if rev > i32::MAX / 10 || rev == i32::MAX / 10 && pop > 7 {
            return 0;
        }
        if rev < i32::MIN / 10 || rev == i32::MIN / 10 && pop < (-8) {
            return 0;
        }
        rev = rev * 10 + pop;
    }
    rev
}
// math
#[test]
fn test1_7() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
}
