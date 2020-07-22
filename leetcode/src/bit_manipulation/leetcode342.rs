// https://leetcode.com/problems/power-of-four/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_power_of_four(num: i32) -> bool {
    num > 0 && num & (num - 1) == 0 && num % 3 == 1
}
// bit_manipulation
#[test]
fn test1_342() {
    assert_eq!(is_power_of_four(16), true);
    assert_eq!(is_power_of_four(5), false);
}
