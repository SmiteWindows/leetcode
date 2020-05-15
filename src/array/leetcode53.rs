// https://leetcode.com/problems/maximum-subarray/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut sum = 0;
    for num in nums {
        if sum > 0 {
            sum += num;
        } else {
            sum = num;
        }
        res = res.max(sum);
    }
    res
}
// divide_and_conquer array dynamic_programming
#[test]
fn test2_53() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
