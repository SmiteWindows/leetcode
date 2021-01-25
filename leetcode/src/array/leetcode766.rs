// https://leetcode-cn.com/problems/toeplitz-matrix/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();
    for i in 0..n - 1 {
        for j in 0..m - 1 {
            if matrix[i][j] != matrix[i + 1][j + 1] {
                return false;
            }
        }
    }
    true
}
// array
#[test]
fn test1_766() {
    use leetcode_prelude::vec2;
    assert_eq!(
        is_toeplitz_matrix(vec2![[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]]),
        true
    );
    assert_eq!(is_toeplitz_matrix(vec2![[1, 2], [2, 2]]), false);
}
