// https://leetcode-cn.com/problems/bitwise-and-of-numbers-range/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
    let mut n = n;
    while n > m {
        n &= n - 1;
    }
    n
}
// bit_manipulation
#[test]
fn test1_201() {
    assert_eq!(range_bitwise_and(5, 7), 4);
    assert_eq!(range_bitwise_and(0, 1), 0);
}
