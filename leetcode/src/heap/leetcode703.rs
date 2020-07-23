// https://leetcode.com/problems/kth-largest-element-in-a-stream/
// Runtime: 8 ms
// Memory Usage: 5.5 MB
use std::{cmp::Reverse, collections::BinaryHeap};
struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut pq = BinaryHeap::new();
        for x in nums {
            pq.push(Reverse(x));
            if pq.len() > k {
                pq.pop();
            }
        }
        Self { pq, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.k {
            self.pq.pop();
        }
        let x = *self.pq.peek().unwrap();
        x.0
    }
}
/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
// heap
#[test]
fn test1_703() {
    let k = 3;
    let nums = vec![4, 5, 8, 2];
    let mut obj = KthLargest::new(k, nums);
    assert_eq!(obj.add(3), 4);
    assert_eq!(obj.add(5), 5);
    assert_eq!(obj.add(10), 5);
    assert_eq!(obj.add(9), 8);
    assert_eq!(obj.add(4), 8);
}
