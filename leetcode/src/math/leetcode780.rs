// https://leetcode-cn.com/problems/reaching-points/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    while sx < tx && sy < ty {
        if tx < ty {
            ty %= tx;
        } else {
            tx %= ty;
        }
    }
    sx == tx && sy <= ty && (ty - sy) % sx == 0 || sy == ty && sx <= tx && (tx - sx) % sy == 0
}
// math
#[test]
fn test1_780() {
    assert_eq!(reaching_points(1, 1, 3, 5), true);
    assert_eq!(reaching_points(1, 1, 2, 2), false);
    assert_eq!(reaching_points(1, 1, 1, 1), true);
}
