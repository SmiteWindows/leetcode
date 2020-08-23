// https://leetcode.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
pub fn min_operations(nums: Vec<i32>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1558() {
    assert_eq!(min_operations(vec![1, 5]), 5);
    assert_eq!(min_operations(vec![2, 2]), 3);
    assert_eq!(min_operations(vec![4, 2, 5]), 6);
    assert_eq!(min_operations(vec![3, 2, 2, 4]), 7);
    assert_eq!(min_operations(vec![2, 4, 8, 16]), 8);
}
