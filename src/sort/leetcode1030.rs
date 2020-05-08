// https://leetcode.com/problems/matrix-cells-in-distance-order/
// Runtime: 20 ms
// Memory Usage: 2.6 MB
pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
    let mut cells = vec![];
    for i in 0..r {
        for j in 0..c {
            cells.push(vec![i, j]);
        }
    }
    cells.sort_unstable_by_key(|v| (v[0] - r0).abs() + (v[1] - c0).abs());
    cells
}
// sort
#[test]
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
