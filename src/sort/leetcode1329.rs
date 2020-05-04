// https://leetcode.com/problems/sort-the-matrix-diagonally/
pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// sort array
#[test]
#[ignore]
fn test1_1329() {
    assert_eq!(
        diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
        vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
    );
}
