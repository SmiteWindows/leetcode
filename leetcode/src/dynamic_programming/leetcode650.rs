// https://leetcode-cn.com/problems/2-keys-keyboard/
// Runtime: 72 ms
// Memory Usage: 2 MB
pub fn min_steps(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    for i in 2..=n {
        dp[i] = i;
        for j in (2..i).rev() {
            if i % j == 0 {
                dp[i] = dp[j] + i / j;
                break;
            }
        }
    }
    dp[n] as i32
}
// dynamic_programming
#[test]
fn test1_650() {
    assert_eq!(min_steps(3), 3);
}
