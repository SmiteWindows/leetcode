// https://leetcode.com/problems/last-stone-weight/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::BinaryHeap;
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut pq = BinaryHeap::from(stones);
    while let Some(a) = pq.pop() {
        if let Some(b) = pq.pop() {
            if a - b != 0 {
                pq.push(a - b);
            }
        } else {
            return a;
        }
    }
    0
}
// greedy heap
#[test]
fn test2_1046() {
    assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
}
