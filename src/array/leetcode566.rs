// https://leetcode.com/problems/reshape-the-matrix/
pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_566() {
    assert_eq!(
        matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
    assert_eq!(
        matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );
}
