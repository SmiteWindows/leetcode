// https://leetcode.com/problems/maximum-sum-circular-subarray/
pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_918() {
    assert_eq!(max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
    assert_eq!(max_subarray_sum_circular(vec![3, -1, 2, -1]), 4);
    assert_eq!(max_subarray_sum_circular(vec![3, -2, 2, -3]), 3);
    assert_eq!(max_subarray_sum_circular(vec![-2, -3, -1]), -1);
}
