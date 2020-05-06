// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1072() {
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]), 1);
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]), 2);
    assert_eq!(
        max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
        2
    );
}
