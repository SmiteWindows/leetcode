// https://leetcode-cn.com/problems/minimum-subsequence-in-non-increasing-order/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let n = nums.len();
    nums.sort_unstable();
    let sum: i32 = nums.iter().sum();
    let mut res = vec![];
    let mut cur = 0;
    for &item in nums.iter().rev() {
        if cur * 2 <= sum {
            res.push(item);
        }
        cur += item;
    }
    res
}
// sort greedy
#[test]
fn test2_1403() {
    assert_eq!(min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
    assert_eq!(min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    assert_eq!(min_subsequence(vec![6]), vec![6]);
}
