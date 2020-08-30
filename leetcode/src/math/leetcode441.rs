// https://leetcode-cn.com/problems/arranging-coins/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn arrange_coins(n: i32) -> i32 {
    (((2 * n as i64) as f64 + 0.25).sqrt() - 0.5).floor() as i32
}
// math binary_search
#[test]
fn test1_441() {
    assert_eq!(arrange_coins(5), 2);
    assert_eq!(arrange_coins(8), 3);
}
