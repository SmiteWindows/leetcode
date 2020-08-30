// https://leetcode-cn.com/problems/valid-perfect-square/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_perfect_square(num: i32) -> bool {
    if num < 2 {
        return true;
    }
    let num = num as i64;
    let mut x = num / 2;
    while x * x > num {
        x = (x + num / x) / 2;
    }
    x * x == num
}
// math binary_search
#[test]
fn test1_367() {
    assert_eq!(is_perfect_square(16), true);
    assert_eq!(is_perfect_square(14), false);
}
