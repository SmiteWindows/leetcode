// https://leetcode-cn.com/problems/minimum-value-to-get-positive-step-by-step-sum/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut start = 1;
    for val in nums.iter() {
        sum += val;
        if start + sum < 1 {
            start = start.max(1 - sum);
        }
    }
    start
}
// array
#[test]
fn test1_1413() {
    assert_eq!(min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    assert_eq!(min_start_value(vec![1, 2]), 1);
    assert_eq!(min_start_value(vec![1, -2, -3]), 5);
}
