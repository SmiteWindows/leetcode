// https://leetcode.com/problems/split-array-into-consecutive-subsequences/
pub fn is_possible(nums: Vec<i32>) -> bool {
    todo!()
}
// heap greedy
#[test]
#[ignore]
fn test1_659() {
    assert_eq!(is_possible(vec![1, 2, 3, 3, 4, 5]), true);
    assert_eq!(is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
    assert_eq!(is_possible(vec![1, 2, 3, 4, 4, 5]), false);
}
