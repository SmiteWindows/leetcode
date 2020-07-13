// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/
pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    todo!()
}
// array sort
#[test]
#[ignore]
fn test1_1508() {
    assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
    assert_eq!(range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
}
