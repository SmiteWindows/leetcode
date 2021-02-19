// https://leetcode-cn.com/problems/nim-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}
// brainteaser minimax
#[test]
fn test1_292() {
    assert_eq!(can_win_nim(4), false);
}
