// https://leetcode-cn.com/problems/maximum-number-of-coins-you-can-get/
// Runtime: 40 ms
// Memory Usage: 3.1 MB
pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable();
    let mut res = 0;
    let n = piles.len() / 3;
    for i in 0..n {
        res += piles[i * 2 + n];
    }
    res
}
// sort
#[test]
fn test1_1561() {
    assert_eq!(max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
    assert_eq!(max_coins(vec![2, 4, 5]), 4);
    assert_eq!(max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
}
