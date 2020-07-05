// https://leetcode.com/problems/count-submatrices-with-all-ones/
pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1504() {
    assert_eq!(
        num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        13
    );
    assert_eq!(
        num_submat(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]]),
        24
    );
    assert_eq!(num_submat(vec![vec![1, 1, 1, 1, 1, 1]]), 21);
    assert_eq!(
        num_submat(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]),
        5
    );
}
