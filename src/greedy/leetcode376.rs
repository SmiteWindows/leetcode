// https://leetcode.com/problems/wiggle-subsequence/
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    todo!()
}
// greedy dynamic_programming
#[test]
#[ignore]
fn test1_376() {
    assert_eq!(wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    assert_eq!(
        wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
        7
    );
    assert_eq!(wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
}
