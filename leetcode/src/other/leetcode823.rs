// https://leetcode-cn.com/problems/binary-trees-with-factors/
// Runtime: 20 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn num_factored_binary_trees(mut a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut dp = vec![1_i64; n];
    let modulo = 1_000_000_007;
    let mut hm = HashMap::new();
    a.sort_unstable();
    let mut res = 0;
    for i in 0..n {
        hm.insert(a[i], i);
        for j in 0..i {
            if a[i] % a[j] == 0 {
                if let Some(&k) = hm.get(&(a[i] / a[j])) {
                    dp[i] += dp[j] * dp[k];
                }
            }
        }
        res = (res + dp[i]) % modulo;
    }
    res as i32
}
#[test]
fn test823() {
    assert_eq!(num_factored_binary_trees(vec![2, 4]), 3);
    assert_eq!(num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
}
