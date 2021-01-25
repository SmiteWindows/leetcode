// https://leetcode-cn.com/problems/transpose-matrix/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = a[0].len();
    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; m];
    for (i, arr_i) in a.iter().enumerate().take(n) {
        for (j, arr_j) in res.iter_mut().enumerate().take(m) {
            arr_j[i] = arr_i[j];
        }
    }
    res
}
// array
#[test]
fn test1_867() {
    use leetcode_prelude::vec2;
    assert_eq!(
        transpose(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
        vec2![[1, 4, 7], [2, 5, 8], [3, 6, 9]]
    );
    assert_eq!(
        transpose(vec2![[1, 2, 3], [4, 5, 6]]),
        vec2![[1, 4], [2, 5], [3, 6]]
    );
}
