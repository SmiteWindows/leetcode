// https://leetcode-cn.com/problems/maximum-subarray/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, std::i32::MIN), |(cur, mx), &num| {
            (std::cmp::max(0, cur + num), std::cmp::max(mx, cur + num))
        })
        .1
}
// divide_and_conquer array dynamic_programming
#[test]
fn test3_53() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
