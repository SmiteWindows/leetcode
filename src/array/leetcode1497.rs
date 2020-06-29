// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/
pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    todo!()
}
// array math greedy
#[test]
#[ignore]
fn test1_1497() {
    assert_eq!(can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5), true);
    assert_eq!(can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
    assert_eq!(can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
    assert_eq!(can_arrange(vec![-10, 10], 2), true);
    assert_eq!(can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3), true);
}
