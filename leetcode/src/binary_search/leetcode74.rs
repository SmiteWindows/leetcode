// https://leetcode-cn.com/problems/search-a-2d-matrix/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut a = vec![];
    for row in matrix {
        for x in row {
            a.push(x);
        }
    }
    a.binary_search(&target).is_ok()
}
// binary_search array
#[test]
fn test1_74() {
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
