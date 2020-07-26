// https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
pub fn min_number_operations(target: Vec<i32>) -> i32 {
    todo!()
}
// segment_tree
#[test]
#[ignore]
fn test1_1526() {
    assert_eq!(min_number_operations(vec![1, 2, 3, 2, 1]), 3);
    assert_eq!(min_number_operations(vec![3, 1, 1, 2]), 4);
    assert_eq!(min_number_operations(vec![3, 1, 5, 4, 2]), 7);
    assert_eq!(min_number_operations(vec![1, 1, 1, 1]), 1);
}
