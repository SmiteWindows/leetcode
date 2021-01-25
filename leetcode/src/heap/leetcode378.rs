// https://leetcode-cn.com/problems/kth-smallest-element-in-a-sorted-matrix/
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
    use leetcode_prelude::vec2;
    assert_eq!(
        kth_smallest(vec2![[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8),
        13
    );
}
