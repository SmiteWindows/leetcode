// https://leetcode.com/problems/delete-and-earn/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let n = 10001;
    let mut sum = vec![0; n];
    let mut dp = vec![0; n];
    for x in nums {
        sum[x as usize] += x;
    }
    dp[1] = sum[1];
    for i in 2..n {
        dp[i] = dp[i - 1].max(sum[i] + dp[i - 2]);
    }
    dp[n - 1]
}
// dynamic_programming
#[test]
fn test1_740() {
    assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
