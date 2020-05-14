// https://leetcode.com/problems/sum-of-square-numbers/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn judge_square_sum(c: i32) -> bool {
    let x = (c as f64).sqrt() as i32;
    for a in 0..=x {
        let bb = c - a * a;
        let b = (bb as f64).sqrt() as i32;
        if bb == b * b {
            return true;
        }
    }
    false
}
// math
#[test]
fn test1_633() {
    assert_eq!(judge_square_sum(5), true);
    assert_eq!(judge_square_sum(3), false);
}
