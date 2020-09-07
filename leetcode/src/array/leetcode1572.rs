// https://leetcode.com/problems/matrix-diagonal-sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j || i + j == n - 1 {
                res += mat[i][j];
            }
        }
    }
    res
}
// array
#[test]
fn test1_1572() {
    use leetcode_prelude::vec2;
    assert_eq!(diagonal_sum(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]]), 25);
    assert_eq!(
        diagonal_sum(vec2![
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]),
        8
    );
    assert_eq!(diagonal_sum(vec2![[5]]), 5);
}
