// https://leetcode.com/problems/combination-sum-iv/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let k = target as usize;
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 1..=target {
        for &j in &nums {
            if i - j >= 0 {
                dp[i as usize] += dp[(i - j) as usize];
            }
        }
    }
    dp[k]
}
// dynamic_programming
#[test]
fn test1_377() {
    assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
}
