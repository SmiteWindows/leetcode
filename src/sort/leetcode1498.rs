// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}
// sliding_window sort
#[test]
#[ignore]
fn test2_1498() {
    assert_eq!(num_subseq(vec![3, 5, 6, 7], 9), 4);
    assert_eq!(num_subseq(vec![3, 3, 6, 8], 10), 6);
    assert_eq!(num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
    assert_eq!(num_subseq(vec![5, 2, 4, 1, 7, 6, 8], 16), 127);
}
