// https://leetcode-cn.com/problems/validate-binary-tree-nodes/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let n = n as usize;
    let mut indegree = vec![0; n];
    let mut outdegree = vec![0; n];
    let mut edge = 0;
    for i in 0..n {
        if left_child[i] != -1 {
            edge += 1;
            outdegree[i] += 1;
            indegree[left_child[i] as usize] += 1;
        }
        if right_child[i] != -1 {
            edge += 1;
            outdegree[i] += 1;
            indegree[right_child[i] as usize] += 1;
        }
    }
    let degree = (0..n).any(|i| {
        let a = n != 1 && indegree[i] == 0 && outdegree[i] == 0;
        let b = indegree[i] > 1;
        let c = outdegree[i] > 2;
        a || b || c
    });
    !degree && edge + 1 == n
}
// graph
#[test]
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
