// https://leetcode.com/problems/new-21-game/
// FIX
pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
    if n == 0 || n > k + w {
        return 1.0;
    }
    let n = n as usize;
    let w = w as usize;
    let k = k as usize;
    let mut dp = vec![0.0; n + 1];
    dp[0] = 1.0;
    let mut sum = 1.0;
    let mut res = 0.0;
    for i in 1..=n {
        dp[i] = sum / w as f64;
        if i < k {
            sum += dp[i];
        } else {
            res += dp[i];
        }
        if i >= w {
            sum -= dp[i - w];
        }
    }
    res
}
// dynamic_programming
#[test]
#[ignore]
fn test1_837() {
    assert_eq!(new21_game(10, 1, 10), 1.00000);
    assert_eq!(new21_game(6, 1, 10), 0.60000);
    assert_eq!(new21_game(21, 17, 10), 0.73278);
}
