// https://leetcode.com/problems/previous-permutation-with-one-swap/
pub fn prev_perm_opt1(a: Vec<i32>) -> Vec<i32> {
    todo!()
}
// greedy array
#[test]
#[ignore]
fn test2_1053() {
    assert_eq!(prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
    assert_eq!(prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
    assert_eq!(prev_perm_opt1(vec![1, 9, 4, 6, 7]), vec![1, 7, 4, 6, 9]);
    assert_eq!(prev_perm_opt1(vec![3, 1, 1, 3]), vec![1, 3, 1, 3]);
}
