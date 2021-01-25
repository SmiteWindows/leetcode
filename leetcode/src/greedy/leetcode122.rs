// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max_profit += prices[i] - prices[i - 1]
        }
    }
    max_profit
}
// greedy array
#[test]
fn test1_122() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
