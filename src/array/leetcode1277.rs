// https://leetcode.com/problems/count-square-submatrices-with-all-ones/
pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// dynamic_programming array
#[test]
#[ignore]
fn test2_1277() {
    assert_eq!(
        count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]),
        15
    );
    assert_eq!(
        count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        7
    );
}
