// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/
pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1005() {
    assert_eq!(largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    assert_eq!(largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    assert_eq!(largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
}
