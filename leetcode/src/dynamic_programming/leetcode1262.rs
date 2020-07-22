// https://leetcode.com/problems/greatest-sum-divisible-by-three/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![vec![0; 3]; 2];
    for i in 0..n {
        for j in 0..3 {
            dp[(i + 1) % 2][j] = dp[(i + 1) % 2][j].max(dp[i % 2][j]);
            let sum = dp[i % 2][j] + nums[i];
            let sum_mod_3 = (sum % 3) as usize;
            dp[(i + 1) % 2][sum_mod_3] = dp[(i + 1) % 2][sum_mod_3].max(sum);
        }
    }
    dp[n % 2][0]
}
// dynamic_programming
#[test]
fn test1_1262() {
    assert_eq!(max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
    assert_eq!(max_sum_div_three(vec![4]), 0);
    assert_eq!(max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
}
