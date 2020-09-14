// https://leetcode-cn.com/problems/valid-boomerang/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    points[0][0] * (points[1][1] - points[2][1])
        + points[1][0] * (points[2][1] - points[0][1])
        + points[2][0] * (points[0][1] - points[1][1])
        != 0
}
// math
#[test]
fn test1_1037() {
    use leetcode_prelude::vec2;
    assert_eq!(is_boomerang(vec2![[1, 1], [2, 3], [3, 2]]), true);
    assert_eq!(is_boomerang(vec2![[1, 1], [2, 2], [3, 3]]), false);
}
