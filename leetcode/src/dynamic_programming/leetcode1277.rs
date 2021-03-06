// https://leetcode-cn.com/problems/count-square-submatrices-with-all-ones/
// Runtime: 12 ms
// Memory Usage: 2.6 MB
pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 1 {
                matrix[i][j] = if i > 0 && j > 0 {
                    1 + [matrix[i - 1][j], matrix[i][j - 1], matrix[i - 1][j - 1]]
                        .iter()
                        .min()
                        .unwrap()
                } else {
                    1
                };
            }
            res += matrix[i][j];
        }
    }
    res
}
// dynamic_programming array
#[test]
fn test1_1277() {
    use leetcode_prelude::vec2;
    assert_eq!(
        count_squares(vec2![[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]),
        15
    );
    assert_eq!(count_squares(vec2![[1, 0, 1], [1, 1, 0], [1, 1, 0]]), 7);
}
