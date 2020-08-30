// https://leetcode-cn.com/problems/minimum-moves-to-equal-array-elements-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn min_moves2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();
    let median = nums[n / 2];
    nums.into_iter().map(|x| (x - median).abs()).sum()
}
// math
#[test]
fn test1_462() {
    assert_eq!(min_moves2(vec![1, 2, 3]), 2);
}
