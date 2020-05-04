// https://leetcode.com/problems/is-graph-bipartite/
pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    todo!()
}
// graph depth_first_search breadth_first_search
#[test]
#[ignore]
fn test2_785() {
    assert_eq!(
        is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
        true
    );
    assert_eq!(
        is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
        false
    );
}
