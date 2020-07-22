// https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search union_find
#[test]
#[ignore]
fn test2_1489() {
    assert_eq!(
        find_critical_and_pseudo_critical_edges(
            5,
            vec![
                vec![0, 1, 1],
                vec![1, 2, 1],
                vec![2, 3, 2],
                vec![0, 3, 2],
                vec![0, 4, 3],
                vec![3, 4, 3],
                vec![1, 4, 6]
            ]
        ),
        vec![vec![0, 1], vec![2, 3, 4, 5]]
    );
    assert_eq!(
        find_critical_and_pseudo_critical_edges(
            4,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]]
        ),
        vec![vec![], vec![0, 1, 2, 3]]
    );
}
