// https://leetcode-cn.com/problems/robot-return-to-origin/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn judge_circle(moves: String) -> bool {
    let mut x = 0;
    let mut y = 0;
    for c in moves.chars() {
        match c {
            'U' => y += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            'R' => x += 1,
            _ => (),
        }
    }
    x == 0 && y == 0
}
// string
#[test]
fn test1_657() {
    assert_eq!(judge_circle("UD".to_string()), true);
    assert_eq!(judge_circle("LL".to_string()), false);
}
