// https://leetcode-cn.com/problems/longest-increasing-subsequence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = Vec::new();
    for x in nums {
        if let Err(i) = dp.binary_search(&x) {
            if i == dp.len() {
                dp.push(x)
            } else {
                dp[i] = x;
            }
        }
    }
    dp.len() as i32
}
// binary_search dynamic_programming
#[test]
fn test2_300() {
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
}
