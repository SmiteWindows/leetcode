// https://leetcode-cn.com/problems/number-of-1-bits/
#![allow(non_snake_case)]
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn hammingWeight(n: u32) -> i32 {
    n.count_ones() as i32
}
// bit_manipulation
#[test]
fn test1_191() {
    assert_eq!(hammingWeight(0b00000000000000000000000000001011), 3);
    assert_eq!(hammingWeight(0b00000000000000000000000010000000), 1);
    assert_eq!(hammingWeight(0b11111111111111111111111111111101), 31);
}
