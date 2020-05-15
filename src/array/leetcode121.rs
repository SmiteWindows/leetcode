// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut mins = vec![0; n];
    let mut prev_min = std::i32::MAX;
    for (i, &price) in prices.iter().enumerate() {
        prev_min = prev_min.min(price);
        mins[i] = prev_min;
    }
    let mut res = 0;
    for (i, &price) in prices.iter().enumerate().skip(1) {
        res = res.max(price - mins[i - 1]);
    }
    res
}
// dynamic_programming array
#[test]
fn test2_121() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
