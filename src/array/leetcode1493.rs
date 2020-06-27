// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1493() {
    assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    assert_eq!(longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]), 4);
    assert_eq!(longest_subarray(vec![0, 0, 0]), 0);
}
