// https://leetcode.com/problems/maximum-subarray/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut max = std::i32::MIN;
    for &val in nums.iter() {
        prev = val.max(prev + val);
        max = max.max(prev);
    }
    max
}
// divide_and_conquer array dynamic_programming
#[test]
fn test3_53() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
