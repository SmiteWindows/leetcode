// https://leetcode-cn.com/problems/coin-change/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let n = (amount + 1) as usize;
    let mut a = vec![-1; n];
    a[0] = 0;
    for i in 1..n {
        for &coin in &coins {
            if coin as usize <= i {
                let j = i - coin as usize;
                if a[j] != -1 {
                    if a[i] == -1 {
                        a[i] = a[j] + 1
                    } else {
                        a[i] = a[i].min(a[j] + 1);
                    }
                }
            }
        }
    }
    a[amount as usize]
}
// dynamic_programming
#[test]
fn test1_322() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(coin_change(vec![2], 3), -1);
}
