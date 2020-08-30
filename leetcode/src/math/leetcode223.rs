// https://leetcode-cn.com/problems/rectangle-area/
#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    let left = a.max(e);
    let right = c.min(g);
    let bottom = b.max(f);
    let top = d.min(h);
    let r1 = (c - a) * (d - b);
    let r2 = (g - e) * (h - f);
    let r3 = (right.max(left) - left) * (top.max(bottom) - bottom);
    r1 + r2 - r3
}
// math
#[test]
fn test1_223() {
    assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
}
