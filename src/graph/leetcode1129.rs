// https://leetcode.com/problems/shortest-path-with-alternating-colors/
pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    todo!()
}
// graph breadth_first_search
#[test]
#[ignore]
fn test1_1129() {
    assert_eq!(
        shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
        vec![0, 1, -1]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
        vec![0, 1, -1]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec![vec![1, 0]], vec![vec![2, 1]]),
        vec![0, -1, -1]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![1, 2]]),
        vec![0, 1, 2]
    );
    assert_eq!(
        shortest_alternating_paths(3, vec![vec![0, 1], vec![0, 2]], vec![vec![1, 0]]),
        vec![0, 1, 1]
    );
}
