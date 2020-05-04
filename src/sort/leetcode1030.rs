// https://leetcode.com/problems/matrix-cells-in-distance-order/
pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
    todo!()
}
// sort
#[test]
#[ignore]
fn test1_1030() {
    assert_eq!(
        all_cells_dist_order(1, 2, 0, 0),
        vec![vec![0, 0], vec![0, 1]]
    );
    assert_eq!(
        all_cells_dist_order(2, 2, 0, 1),
        vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]
    );
    assert_eq!(
        all_cells_dist_order(2, 3, 1, 2),
        vec![
            vec![1, 2],
            vec![0, 2],
            vec![1, 1],
            vec![0, 1],
            vec![1, 0],
            vec![0, 0]
        ]
    );
}
