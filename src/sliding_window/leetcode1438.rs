// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    todo!()
}
// array sliding_window
#[test]
#[ignore]
fn test2_1438() {
    assert_eq!(longest_subarray(vec![8, 2, 4, 7], 4), 2);
    assert_eq!(longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
    assert_eq!(longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
}
