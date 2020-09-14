// https://leetcode-cn.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search union_find
#[test]
#[ignore]
fn test1_1489() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_critical_and_pseudo_critical_edges(
            5,
            vec2![
                [0, 1, 1],
                [1, 2, 1],
                [2, 3, 2],
                [0, 3, 2],
                [0, 4, 3],
                [3, 4, 3],
                [1, 4, 6]
            ]
        ),
        vec2![[0, 1], [2, 3, 4, 5]]
    );
    assert_eq!(
        find_critical_and_pseudo_critical_edges(
            4,
            vec2![[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]
        ),
        vec2![[], [0, 1, 2, 3]]
    );
}
