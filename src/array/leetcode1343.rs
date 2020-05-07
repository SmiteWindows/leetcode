// https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1343() {
    assert_eq!(num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
    assert_eq!(num_of_subarrays(vec![1, 1, 1, 1, 1], 1, 0), 5);
    assert_eq!(
        num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
        6
    );
    assert_eq!(num_of_subarrays(vec![7, 7, 7, 7, 7, 7, 7], 7, 7), 1);
    assert_eq!(num_of_subarrays(vec![4, 4, 4, 4], 4, 1), 1);
}
