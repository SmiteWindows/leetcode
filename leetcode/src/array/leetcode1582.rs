// https://leetcode-cn.com/problems/special-positions-in-a-binary-matrix/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let mut rows = vec![0; n];
    let mut cols = vec![0; m];
    for i in 0..n {
        for (j, col) in cols.iter_mut().enumerate().take(m) {
            if mat[i][j] == 1 {
                rows[i] += 1;
                *col += 1;
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        for (j, &col) in cols.iter().enumerate().take(m) {
            if mat[i][j] == 1 && rows[i] == 1 && col == 1 {
                res += 1;
            }
        }
    }
    res
}
// array
#[test]
fn test1_1582() {
    use leetcode_prelude::vec2;
    assert_eq!(num_special(vec2![[1, 0, 0], [0, 0, 1], [1, 0, 0]]), 1);
    assert_eq!(num_special(vec2![[1, 0, 0], [0, 1, 0], [0, 0, 1]]), 3);
    assert_eq!(
        num_special(vec2![
            [0, 0, 0, 1],
            [1, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]),
        2
    );
    assert_eq!(
        num_special(vec2![
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 1]
        ]),
        3
    );
}
