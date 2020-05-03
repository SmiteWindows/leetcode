// https://leetcode.com/problems/rectangle-area-ii/
pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// segment_tree line_sweep
#[test]
#[ignore]
fn test2_850() {
    assert_eq!(
        rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]),
        6
    );
    assert_eq!(rectangle_area(vec![vec![0, 0, 1000000000, 1000000000]]), 49);
}
