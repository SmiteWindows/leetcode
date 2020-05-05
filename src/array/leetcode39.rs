// https://leetcode.com/problems/combination-sum/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}
// array backtracking
#[test]
#[ignore]
fn test2_39() {
    assert_eq!(
        combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![7], vec![2, 2, 3]]
    );
    assert_eq!(
        combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}
