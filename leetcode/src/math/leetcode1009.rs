// https://leetcode-cn.com/problems/complement-of-base-10-integer/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let mut mask = !0;
    while mask & n != 0 {
        mask <<= 1;
    }
    mask = !mask;
    mask ^ n
}
// math
#[test]
fn test1_1009() {
    assert_eq!(bitwise_complement(5), 2);
    assert_eq!(bitwise_complement(7), 0);
    assert_eq!(bitwise_complement(10), 5);
}
