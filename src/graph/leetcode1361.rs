// https://leetcode.com/problems/validate-binary-tree-nodes/
pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1361() {
    assert_eq!(
        validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]),
        true
    );
    assert_eq!(
        validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]),
        false
    );
    assert_eq!(
        validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]),
        false
    );
    assert_eq!(
        validate_binary_tree_nodes(6, vec![1, -1, -1, 4, -1, -1], vec![2, -1, -1, 5, -1, -1]),
        false
    );
}
