// https://leetcode.com/problems/longest-arithmetic-sequence/
// Runtime: 248 ms
// Memory Usage: 27.3 MB
use std::collections::HashMap;
pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
    let mut max = 1;
    let n = a.len();
    let mut dp = vec![HashMap::new(); n];
    for i in 0..n {
        for j in 0..i {
            let diff = a[i] - a[j];
            let len = dp[j].get(&diff).unwrap_or(&1) + 1;
            dp[i].insert(diff, len);
            max = i32::max(len, max);
        }
    }
    max
}
// dynamic_programming
#[test]
fn test1_1027() {
    assert_eq!(longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    assert_eq!(longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    assert_eq!(longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]), 4);
}
