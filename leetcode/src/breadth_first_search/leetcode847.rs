// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// dynamic_programming breadth_first_search
#[test]
#[ignore]
fn test2_847() {
    assert_eq!(
        shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]),
        4
    );
    assert_eq!(
        shortest_path_length(vec![
            vec![1],
            vec![0, 2, 4],
            vec![1, 3, 4],
            vec![2],
            vec![1, 2]
        ]),
        4
    );
}
