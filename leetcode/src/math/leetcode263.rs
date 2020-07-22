// https://leetcode.com/problems/ugly-number/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_ugly(num: i32) -> bool {
    if num < 1 {
        return false;
    }
    let mut num = num;
    while num % 2 == 0 {
        num /= 2;
    }
    while num % 3 == 0 {
        num /= 3;
    }
    while num % 5 == 0 {
        num /= 5;
    }
    num == 1
}
// math
#[test]
fn test1_263() {
    assert_eq!(is_ugly(6), true);
    assert_eq!(is_ugly(8), true);
    assert_eq!(is_ugly(14), false);
}
