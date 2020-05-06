// https://leetcode.com/problems/largest-plus-sign/
pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_764() {
    assert_eq!(order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
    assert_eq!(order_of_largest_plus_sign(2, vec![vec![]]), 1);
    assert_eq!(order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
}
