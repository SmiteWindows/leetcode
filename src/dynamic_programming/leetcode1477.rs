// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1477() {
    assert_eq!(min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    assert_eq!(min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    assert_eq!(min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6), -1);
    assert_eq!(min_sum_of_lengths(vec![5, 5, 4, 4, 5], 3), -1);
    assert_eq!(min_sum_of_lengths(vec![3, 1, 1, 1, 5, 1, 2, 1], 3), 3);
}
