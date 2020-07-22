// https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/
pub fn reachable_nodes(edges: Vec<Vec<i32>>, m: i32, n: i32) -> i32 {
    todo!()
}
// heap
#[test]
#[ignore]
fn test1_882() {
    assert_eq!(
        reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
        13
    );
    assert_eq!(
        reachable_nodes(
            vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
            10,
            4
        ),
        23
    );
}
