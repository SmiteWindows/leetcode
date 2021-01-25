// https://leetcode-cn.com/problems/largest-triangle-area/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut res: f64 = 0_f64;
    for i in &points {
        for j in &points {
            for k in &points {
                let area = (i[0] * j[1] + j[0] * k[1] + k[0] * i[1]
                    - j[0] * i[1]
                    - k[0] * j[1]
                    - i[0] * k[1])
                    .abs();
                let area = area as f64 / 2_f64;
                res = res.max(area);
            }
        }
    }
    res
}
// math
#[test]
fn test1_812() {
    use leetcode_prelude::{assert_approx_eq, vec2};
    assert_approx_eq!(
        largest_triangle_area(vec2![[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]]),
        2.0
    );
}
