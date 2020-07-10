// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
// Runtime: 20 ms
// Memory Usage: 2.8 MB
pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let n = prices.len();
    let mut cash = 0;
    let mut hold = -prices[0];
    for i in 1..n {
        cash = cash.max(hold + prices[i] - fee);
        hold = hold.max(cash - prices[i]);
    }
    cash
}
// greedy array dynamic_programming
#[test]
fn test2_714() {
    assert_eq!(max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
}
