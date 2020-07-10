// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
// Runtime: 96 ms
// Memory Usage: 2.7 MB
pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            let count = matrix[i]
                .iter()
                .zip(matrix[j].iter())
                .map(|(x, y)| if x == y { 1 } else { 0 })
                .sum::<usize>();
            if count == 0 || count == m {
                res[i] += 1;
                res[j] += 1;
            }
        }
    }
    *res.iter().max().unwrap()
}
// hash_table
#[test]
fn test1_1072() {
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]), 1);
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]), 2);
    assert_eq!(
        max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
        2
    );
}
