// https://leetcode-cn.com/problems/palindrome-number/
// Runtime: 4 ms
// Memory Usage: 2 MB
pub fn is_palindrome(x: i32) -> bool {
    let mut x = x;
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut rev = 0;
    while x > rev {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    x == rev || x == rev / 10
}
// math
#[test]
fn test1_9() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
}
