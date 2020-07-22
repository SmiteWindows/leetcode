// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
// Runtime: 0 ms
// Memory Usage: 2 MB
const MOD: i64 = 1_000_000_007;
pub fn num_of_ways(n: i32) -> i32 {
    let n = n as usize;
    let mut a121 = 6_i64;
    let mut a123 = 6_i64;
    for _ in 1..n {
        let b121 = a121 * 3 + a123 * 2;
        let b123 = a121 * 2 + a123 * 2;
        a121 = b121 % MOD;
        a123 = b123 % MOD;
    }
    ((a121 + a123) % MOD) as i32
}
// dynamic_programming
#[test]
fn test1_1411() {
    assert_eq!(num_of_ways(1), 12);
    assert_eq!(num_of_ways(2), 54);
    assert_eq!(num_of_ways(3), 246);
    assert_eq!(num_of_ways(7), 106494);
    assert_eq!(num_of_ways(5000), 30228214);
}
