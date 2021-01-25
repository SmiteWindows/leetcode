// https://leetcode-cn.com/problems/reachable-nodes-in-subdivided-graph/
pub fn reachable_nodes(edges: Vec<Vec<i32>>, m: i32, n: i32) -> i32 {
    todo!()
}
// heap
#[test]
#[ignore]
fn test1_882() {
    use leetcode_prelude::vec2;
    assert_eq!(
        reachable_nodes(vec2![[0, 1, 10], [0, 2, 1], [1, 2, 2]], 6, 3),
        13
    );
    assert_eq!(
        reachable_nodes(vec2![[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]], 10, 4),
        23
    );
}
