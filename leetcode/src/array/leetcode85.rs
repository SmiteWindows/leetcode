// https://leetcode-cn.com/problems/maximal-rectangle/
// Runtime: 8 ms
// Memory Usage: 4.8 MB
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let n = matrix.len();
    if n == 0 {
        return 0;
    }
    let m = matrix[0].len();
    let mut left = vec![0; m];
    let mut right = vec![m; m];
    let mut height = vec![0; m];
    let mut res = 0;
    for mi in matrix.iter().take(n) {
        let mut l = 0;
        let mut r = m;
        for (j, hj) in height.iter_mut().enumerate().take(m) {
            if mi[j] == '1' {
                *hj += 1;
            } else {
                *hj = 0;
            }
        }
        for (j, lj) in left.iter_mut().enumerate().take(m) {
            if mi[j] == '1' {
                *lj = l.max(*lj);
            } else {
                *lj = 0;
                l = j + 1;
            }
        }
        for j in (0..m).rev() {
            if mi[j] == '1' {
                right[j] = right[j].min(r);
            } else {
                right[j] = m;
                r = j;
            }
        }
        for j in 0..m {
            res = res.max((right[j] - left[j]) * height[j]);
        }
    }
    res as i32
}
// stack array hash_table dynamic_programming
#[test]
fn test2_85() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        maximal_rectangle(vec2_char![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ]),
        6
    );
}
