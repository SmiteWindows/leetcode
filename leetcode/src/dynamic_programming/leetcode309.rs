// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n == 0 {
        return 0;
    }
    let mut buy = vec![0; n];
    let mut sell = vec![0; n];
    let mut rest = vec![0; n];
    sell[0] = i32::MIN;
    buy[0] = -prices[0];
    for i in 1..n {
        buy[i] = buy[i - 1].max(rest[i - 1] - prices[i]);
        sell[i] = sell[i - 1].max(buy[i - 1] + prices[i]);
        rest[i] = rest[i - 1].max(sell[i - 1]);
    }
    sell[n - 1].max(rest[n - 1])
}
// dynamic_programming
#[test]
fn test1_309() {
    assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
}
