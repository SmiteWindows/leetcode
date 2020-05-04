// https://leetcode.com/problems/find-eventual-safe-states/
pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// graph depth_first_search
#[test]
#[ignore]
fn test1_802() {
    assert_eq!(
        eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![]
        ]),
        vec![2, 4, 5, 6]
    );
}
