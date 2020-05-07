// https://leetcode.com/problems/shortest-path-in-binary-matrix/
pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1091() {
    assert_eq!(shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]), 2);
    assert_eq!(
        shortest_path_binary_matrix(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]),
        4
    );
}
