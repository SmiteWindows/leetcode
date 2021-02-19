// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv/
// Runtime: 52 ms
// Memory Usage: 2.2 MB
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut k = k as usize;
    k = k.min(n / 2);
    if k == 0 {
        return 0;
    }
    let mut min_costs = vec![i32::MAX; k];
    let mut max_profits = vec![0; k];
    for price in prices {
        min_costs[0] = min_costs[0].min(price);
        max_profits[0] = max_profits[0].max(price - min_costs[0]);
        for i in 1..k {
            min_costs[i] = min_costs[i].min(price - max_profits[i - 1]);
            max_profits[i] = max_profits[i].max(price - min_costs[i]);
        }
    }
    max_profits[k - 1] as i32
}
// dynamic_programming
#[test]
fn test1_188() {
    assert_eq!(max_profit(2, vec![2, 4, 1]), 2);
    assert_eq!(max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
}
