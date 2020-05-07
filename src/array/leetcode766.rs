// https://leetcode.com/problems/toeplitz-matrix/
pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_766() {
    assert_eq!(
        is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
        true
    );
    assert_eq!(is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]), false);
}
