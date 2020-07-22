// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
// Runtime: 0 ms
// Memory Usage: 2 MB
const MOD: i64 = 1_000_000_007;
pub fn count_orders(n: i32) -> i32 {
    let n = n as i64;
    let mut res = 1_i64;
    for i in 1..=n {
        res *= i * 2 - 1;
        res %= MOD;
        res *= i;
        res %= MOD;
    }
    res as i32
}
// math dynamic_programming
#[test]
fn test1_1359() {
    assert_eq!(count_orders(1), 1);
    assert_eq!(count_orders(2), 6);
    assert_eq!(count_orders(3), 90);
}
