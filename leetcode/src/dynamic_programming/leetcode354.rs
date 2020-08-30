// https://leetcode-cn.com/problems/russian-doll-envelopes/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
use std::cmp::Reverse;
pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    let n = envelopes.len();
    envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
    let mut dp = vec![];
    for envelope in envelopes.iter().take(n) {
        let height = envelope[1];
        if let Err(p) = dp.binary_search(&height) {
            if p == dp.len() {
                dp.push(height);
            } else {
                dp[p] = height;
            }
        }
    }
    dp.len() as i32
}
// binary_search dynamic_programming
#[test]
fn test2_354() {
    assert_eq!(
        max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
        3
    );
}
