// https://leetcode-cn.com/problems/top-k-frequent-elements/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut pq = BinaryHeap::new();
    for x in nums {
        *hm.entry(x).or_default() += 1;
    }
    for (x, f) in hm {
        pq.push((Reverse(f), x));
        if pq.len() > k {
            pq.pop();
        }
    }
    pq.into_iter().map(|p| p.1).rev().collect()
}
// hash_table heap
#[test]
fn test2_347() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
}
