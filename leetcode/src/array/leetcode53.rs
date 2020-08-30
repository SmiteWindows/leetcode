// https://leetcode-cn.com/problems/maximum-subarray/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .scan(-1, |s, x| {
            *s = if *s < 0 { x } else { *s + x };
            Some(*s)
        })
        .max()
        .unwrap()
}
// divide_and_conquer array dynamic_programming
#[test]
fn test2_53() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
