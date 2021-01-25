// https://leetcode-cn.com/problems/sum-of-two-integers/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn get_sum(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_sum(a ^ b, (a & b) << 1)
    }
}
// bit_manipulation
#[test]
fn test1_371() {
    assert_eq!(get_sum(1, 2), 3);
    assert_eq!(get_sum(-2, 3), 1);
}
