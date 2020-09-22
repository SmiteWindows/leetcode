// https://leetcode.com/problems/make-sum-divisible-by-p/
pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    todo!()
}
// array binary_search
#[test]
#[ignore]
fn test1_1590() {
    assert_eq!(min_subarray(vec![3, 1, 4, 2], 6), 1);
    assert_eq!(min_subarray(vec![6, 3, 5, 2], 9), 2);
    assert_eq!(min_subarray(vec![1, 2, 3], 3), 0);
    assert_eq!(min_subarray(vec![1, 2, 3], 7), -1);
    assert_eq!(min_subarray(vec![1000000000, 1000000000, 1000000000], 3), 0);
}
