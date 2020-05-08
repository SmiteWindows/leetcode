// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cmp::Reverse, collections::BinaryHeap};
pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
    let reverse: Vec<Reverse<i32>> = a.into_iter().map(Reverse).collect();
    let mut pq = BinaryHeap::from(reverse);
    let mut k = k;
    while k > 0 {
        if let Some(min) = pq.pop() {
            pq.push(Reverse(-min.0));
        }
        k -= 1;
    }
    pq.into_iter().map(|x| x.0).sum()
}
// greedy
#[test]
fn test1_1005() {
    assert_eq!(largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    assert_eq!(largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    assert_eq!(largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
}
