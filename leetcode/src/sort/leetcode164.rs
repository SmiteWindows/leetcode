// https://leetcode-cn.com/problems/maximum-gap/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    nums.sort_unstable();
    let mut res = 0;
    for i in 1..n {
        res = res.max(nums[i] - nums[i - 1]);
    }
    res
}
// sort
#[test]
fn test1_164() {
    assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
    assert_eq!(maximum_gap(vec![10]), 0);
}
