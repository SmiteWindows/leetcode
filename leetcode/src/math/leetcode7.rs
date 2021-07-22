// https://leetcode-cn.com/problems/reverse-integer/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn reverse(x: i32) -> i32 {
    fn helper(mut n: i32) -> Option<i32> {
        let mut res = 0i32;
        while n.abs() != 0 {
            res = res.checked_mul(10)?.checked_add(n % 10)?;
            n /= 10;
        }
        Some(res)
    }
    helper(x).unwrap_or_default()
}
// math
#[test]
fn test1_7() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
}
