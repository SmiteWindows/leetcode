// https://leetcode.com/problems/transpose-matrix/
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
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}
