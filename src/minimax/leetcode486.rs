// https://leetcode.com/problems/predict-the-winner/
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    todo!()
}
// dynamic_programming minimax
#[test]
#[ignore]
fn test1_486() {
    assert_eq!(predict_the_winner(vec![1, 5, 2]), false);
    assert_eq!(predict_the_winner(vec![1, 5, 233, 7]), true);
}
