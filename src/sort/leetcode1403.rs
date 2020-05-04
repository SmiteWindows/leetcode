// https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/
pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}
// sort greedy
#[test]
#[ignore]
fn test1_1403() {
    assert_eq!(min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
    assert_eq!(min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    assert_eq!(min_subsequence(vec![6]), vec![6]);
}
