// https://leetcode.com/problems/01-matrix/
pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search breadth_first_search
#[test]
#[ignore]
fn test1_542() {
    assert_eq!(
        update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    );
    assert_eq!(
        update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
    );
}
