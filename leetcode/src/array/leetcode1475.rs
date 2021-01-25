// https://leetcode-cn.com/problems/final-prices-with-a-special-discount-in-a-shop/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let mut stack = vec![];
    let mut res = prices.to_vec();
    for i in 0..n {
        while let Some(&j) = stack.last() {
            if prices[i] > prices[j] {
                break;
            } else {
                stack.pop();
                res[j] = prices[j] - prices[i];
            }
        }
        stack.push(i);
    }
    res
}
// array
#[test]
fn test1_1475() {
    assert_eq!(final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
    assert_eq!(final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    assert_eq!(final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
}
