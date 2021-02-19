// https://leetcode-cn.com/problems/rectangle-area-ii/
pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// segment_tree line_sweep
#[test]
#[ignore]
fn test2_850() {
    use leetcode_prelude::vec2;
    assert_eq!(
        rectangle_area(vec2![[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]]),
        6
    );
    assert_eq!(rectangle_area(vec2![[0, 0, 1000000000, 1000000000]]), 49);
}
