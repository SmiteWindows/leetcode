// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test1_1351() {
    assert_eq!(
        count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ]),
        8
    );
    assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    assert_eq!(count_negatives(vec![vec![1, -1], vec![-1, -1]]), 3);
    assert_eq!(count_negatives(vec![vec![-1]]), 1);
}
