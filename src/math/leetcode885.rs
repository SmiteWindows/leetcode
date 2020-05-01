// https://leetcode.com/problems/spiral-matrix-iii/
pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_885() {
    assert_eq!(
        spiral_matrix_iii(1, 4, 0, 0),
        vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
    );
    assert_eq!(
        spiral_matrix_iii(5, 6, 1, 4),
        vec![
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![3, 4],
            vec![3, 3],
            vec![3, 2],
            vec![2, 2],
            vec![1, 2],
            vec![0, 2],
            vec![4, 5],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![4, 1],
            vec![3, 1],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![4, 0],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0]
        ]
    );
}
