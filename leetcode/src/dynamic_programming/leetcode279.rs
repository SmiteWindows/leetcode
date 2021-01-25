// https://leetcode-cn.com/problems/perfect-squares/
// Runtime: 32 ms
// Memory Usage: 2.2 MB
pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut j = 1;
        while j * j <= i {
            dp[i] = dp[i].min(dp[i - j * j] + 1);
            j += 1;
        }
    }
    dp[n] as i32
}
// math breadth_first_search dynamic_programming
#[test]
fn test2_279() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
