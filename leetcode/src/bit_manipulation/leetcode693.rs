// https://leetcode-cn.com/problems/binary-number-with-alternating-bits/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn has_alternating_bits(n: i32) -> bool {
    let x = (n >> 1) ^ n;
    (x + 1) & x == 0
}
// bit_manipulation
#[test]
fn test1_693() {
    assert_eq!(has_alternating_bits(5), true);
    assert_eq!(has_alternating_bits(7), false);
    assert_eq!(has_alternating_bits(11), false);
    assert_eq!(has_alternating_bits(10), true);
}
