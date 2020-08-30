// https://leetcode-cn.com/problems/divisor-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}
// math dynamic_programming
#[test]
fn test1_1025() {
    assert_eq!(divisor_game(2), true);
    assert_eq!(divisor_game(3), false);
}
