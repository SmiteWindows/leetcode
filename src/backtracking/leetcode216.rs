// https://leetcode.com/problems/combination-sum-iii/
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    todo!()
}
// backtracking array
#[test]
#[ignore]
fn test1_216() {
    assert_eq!(combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    assert_eq!(
        combination_sum3(3, 9),
        vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
    );
}
