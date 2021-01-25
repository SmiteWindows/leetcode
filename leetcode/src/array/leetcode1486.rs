// https://leetcode-cn.com/problems/xor-operation-in-an-array/
pub fn xor_operation(n: i32, start: i32) -> i32 {
    (start..)
        .step_by(2)
        .take(n as usize)
        .fold(0, |acc, x| acc ^ x)
}
// Runtime: 0 ms
// Memory Usage: 2 MB
// ✔
// array bit_manipulation
#[test]
fn test1_1486() {
    assert_eq!(xor_operation(5, 0), 8);
    assert_eq!(xor_operation(4, 3), 8);
    assert_eq!(xor_operation(1, 7), 7);
    assert_eq!(xor_operation(10, 5), 2);
}
