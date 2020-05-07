// https://leetcode.com/problems/transpose-matrix/
pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_867() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}
