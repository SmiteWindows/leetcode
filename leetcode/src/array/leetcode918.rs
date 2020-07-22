// https://leetcode.com/problems/maximum-sum-circular-subarray/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
    let sum = a.iter().sum::<i32>();
    let mut prev_min = 0;
    let mut prev_max = 0;
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &ai in &a {
        prev_min = ai.min(prev_min + ai);
        min = min.min(prev_min);
        prev_max = ai.max(prev_max + ai);
        max = max.max(prev_max);
    }
    if max < 0 {
        max
    } else {
        max.max(sum - min)
    }
}
// array
#[test]
fn test1_918() {
    assert_eq!(max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
    assert_eq!(max_subarray_sum_circular(vec![3, -1, 2, -1]), 4);
    assert_eq!(max_subarray_sum_circular(vec![3, -2, 2, -3]), 3);
    assert_eq!(max_subarray_sum_circular(vec![-2, -3, -1]), -1);
}
