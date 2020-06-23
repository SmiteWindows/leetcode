// https://leetcode.com/problems/coin-change-2/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let n = amount + 1;
    let mut dp = vec![0; n];
    dp[0] = 1;
    for coin in coins {
        let mut sum = coin as usize;
        while sum <= amount {
            dp[sum] += dp[sum - coin as usize];
            sum += 1;
        }
    }
    dp[amount]
}
#[test]
fn test518() {
    assert_eq!(change(5, vec![1, 2, 5]), 4);
    assert_eq!(change(3, vec![2]), 0);
    assert_eq!(change(10, vec![10]), 1);
}
