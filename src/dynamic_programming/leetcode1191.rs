// https://leetcode.com/problems/k-concatenation-maximum-sum/
pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1191() {
    assert_eq!(k_concatenation_max_sum(vec![1, 2], 3), 9);
    assert_eq!(k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
    assert_eq!(k_concatenation_max_sum(vec![-1, -2], 7), 0);
}
