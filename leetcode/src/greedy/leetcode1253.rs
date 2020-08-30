// https://leetcode-cn.com/problems/reconstruct-a-2-row-binary-matrix/
pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// math greedy
#[test]
#[ignore]
fn test2_1253() {
    assert_eq!(
        reconstruct_matrix(2, 1, vec![1, 1, 1]),
        vec![vec![1, 1, 0], vec![0, 0, 1]]
    );
    assert_eq!(reconstruct_matrix(2, 3, vec![2, 2, 1, 1]), vec![vec![]]);
    assert_eq!(
        reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
        vec![
            vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
        ]
    );
}
