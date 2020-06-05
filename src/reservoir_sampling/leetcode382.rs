// https://leetcode.com/problems/linked-list-random-node/
// Runtime: 8 ms
// Memory Usage: 3.6 MB
struct Solution {
    head: Option<Box<ListNode>>,
    rng: ThreadRng,
}
use rand::prelude::*;
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
    Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        let rng = thread_rng();
        Solution { head, rng }
    }

    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let mut cur = &self.head;
        let mut res = 0;
        let mut count = 0;
        while let Some(node) = cur {
            let val = node.val;
            count += 1;
            if self.rng.gen_range(0, count) == 0 {
                res = val;
            }
            cur = &node.next;
        }
        res
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
// reservoir_sampling
#[test]
fn test1_382() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let mut obj = Solution::new(head);
    for _ in 0..100 {
        let res = obj.get_random();
        match res {
            1 => print!("1"),
            2 => print!("2"),
            3 => print!("3"),
            _ => panic!(),
        }
    }
}
