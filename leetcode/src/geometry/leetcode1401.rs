// https://leetcode-cn.com/problems/circle-and-rectangle-overlapping/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> bool {
    let dx = x_center.my_clamp(x1, x2) - x_center;
    let dy = y_center.my_clamp(y1, y2) - y_center;
    dx * dx + dy * dy <= radius * radius
}

trait MyClamp {
    fn my_clamp(self, min: i32, max: i32) -> i32;
}

impl MyClamp for i32 {
    fn my_clamp(self, min: i32, max: i32) -> i32 {
        self.max(min).min(max)
    }
}
// geometry
#[test]
fn test1_1401() {
    assert_eq!(check_overlap(1, 0, 0, 1, -1, 3, 1), true);
    assert_eq!(check_overlap(1, 0, 0, -1, 0, 0, 1), true);
    assert_eq!(check_overlap(1, 1, 1, -3, -3, 3, 3), true);
    assert_eq!(check_overlap(1, 1, 1, 1, -3, 2, -1), false);
}
