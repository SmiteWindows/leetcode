// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/
pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    todo!()
}
// math dynamic_programming
#[test]
#[ignore]
fn test2_1218() {
    assert_eq!(longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    assert_eq!(longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    assert_eq!(longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
}
