// https://leetcode-cn.com/problems/search-a-2d-matrix/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut a = Vec::new();
    for row in matrix {
        for x in row {
            a.push(x);
        }
    }
    a.binary_search(&target).is_ok()
}
// binary_search array
#[test]
fn test2_74() {
    use leetcode_prelude::vec2;
    assert_eq!(
        search_matrix(vec2![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]], 3),
        true
    );
    assert_eq!(
        search_matrix(vec2![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]], 13),
        false
    );
}
