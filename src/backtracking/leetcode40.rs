// https://leetcode.com/problems/combination-sum-ii/
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}
// array backtracking
#[test]
#[ignore]
fn test1_40() {
    assert_eq!(
        combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]]
    );
    assert_eq!(
        combination_sum2(vec![2, 5, 2, 1, 2], 5),
        vec![vec![1, 2, 2], vec![5]]
    );
}
