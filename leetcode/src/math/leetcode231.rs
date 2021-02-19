// https://leetcode-cn.com/problems/power-of-two/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_power_of_two(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    let x = n as i64;
    x & (x - 1) == 0
}
// bit_manipulation math
#[test]
fn test1_231() {
    assert_eq!(is_power_of_two(1), true);
    assert_eq!(is_power_of_two(16), true);
    assert_eq!(is_power_of_two(218), false);
}
