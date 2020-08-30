// https://leetcode-cn.com/problems/top-k-frequent-elements/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
use std::{cmp::Reverse, collections::HashMap};
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    let mut counts = HashMap::new();
    for n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }
    let mut tuples = counts.iter().collect::<Vec<_>>();
    tuples.sort_by_key(|t| Reverse(t.1));
    tuples.iter().map(|t| *t.0).take(k as usize).collect()
}
// hash_table heap
#[test]
fn test1_347() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
}
