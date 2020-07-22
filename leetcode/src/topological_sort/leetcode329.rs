// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search topological_sort memoization
#[test]
#[ignore]
fn test1_329() {
    assert_eq!(
        longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    );
    assert_eq!(
        longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
        4
    );
}
