// https://leetcode.com/problems/surface-area-of-3d-shapes/
pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// math geometry
#[test]
#[ignore]
fn test1_892() {
    assert_eq!(surface_area(vec![vec![2]]), 10);
    assert_eq!(surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(surface_area(vec![vec![1, 0], vec![0, 2]]), 16);
    assert_eq!(
        surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        32
    );
    assert_eq!(
        surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
        46
    );
}
