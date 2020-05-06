// https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    todo!()
}
// greedy array
#[test]
#[ignore]
fn test2_1296() {
    assert_eq!(is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4), true);
    assert_eq!(
        is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3),
        true
    );
    assert_eq!(is_possible_divide(vec![3, 3, 2, 2, 1, 1], 3), true);
    assert_eq!(is_possible_divide(vec![1, 2, 3, 4], 3), false);
}
