// https://leetcode.com/problems/search-a-2d-matrix/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test2_74() {
    assert_eq!(
        search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            3
        ),
        true
    );
    assert_eq!(
        search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            13
        ),
        false
    );
}
