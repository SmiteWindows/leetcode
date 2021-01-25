// https://leetcode-cn.com/problems/elimination-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn last_remaining(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        2 * (1 + n / 2 - last_remaining(n / 2))
    }
}
#[test]
fn test390() {
    assert_eq!(last_remaining(9), 6);
}
