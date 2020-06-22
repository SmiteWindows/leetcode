// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
// Runtime: 4 ms
// Memory Usage: 2.7 MB
use std::collections::BinaryHeap;
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut pq = BinaryHeap::new();
    for row in matrix {
        for x in row {
            pq.push(x);
            if pq.len() > k as usize {
                pq.pop();
            }
        }
    }
    pq.pop().unwrap()
}
// binary_search heap
#[test]
fn test2_378() {
    assert_eq!(
        kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
        13
    );
}
