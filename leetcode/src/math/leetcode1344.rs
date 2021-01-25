// https://leetcode-cn.com/problems/angle-between-hands-of-a-clock/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let h = ((hour % 12) as f64 + (minutes as f64 / 60.0)) * 30.0;
    let m = minutes as f64 * 6.0;
    let a = (h - m).abs();
    if a > 180.0 {
        360.0 - a
    } else {
        a
    }
}
// math
#[test]
fn test1_1344() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(angle_clock(12, 30), 165.0);
    assert_approx_eq!(angle_clock(3, 30), 75.0);
    assert_approx_eq!(angle_clock(3, 15), 7.5);
    assert_approx_eq!(angle_clock(4, 50), 155.0);
    assert_approx_eq!(angle_clock(12, 0), 0.0);
}
