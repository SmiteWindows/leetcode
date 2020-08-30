// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut t1_cost = i32::MAX;
    let mut t2_cost = i32::MAX;
    let mut t1_profit = 0;
    let mut t2_profit = 0;
    for x in prices {
        t1_cost = t1_cost.min(x);
        t1_profit = t1_profit.max(x - t1_cost);
        t2_cost = t2_cost.min(x - t1_profit);
        t2_profit = t2_profit.max(x - t2_cost);
    }
    t2_profit
}
// dynamic_programming array
#[test]
fn test1_123() {
    assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
