// https://leetcode.com/problems/projection-area-of-3d-shapes/
pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_883() {
    assert_eq!(projection_area(vec![vec![2]]), 5);
    assert_eq!(projection_area(vec![vec![1, 2]], vec![3, 4]), 17);
    assert_eq!(projection_area(vec![vec![1, 0]], vec![0, 2]), 8);
    assert_eq!(
        projection_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        14
    );
    assert_eq!(
        projection_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
        21
    );
}
