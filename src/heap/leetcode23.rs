// https://leetcode.com/problems/merge-k-sorted-lists/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        Some(other.val.cmp(&self.val))
    }
}
use std::{cmp::Ordering, collections::BinaryHeap};
// Runtime: 4 ms
// Memory Usage: 3 MB
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut h: BinaryHeap<Option<Box<ListNode>>> = BinaryHeap::with_capacity(lists.len());
    for x in lists.iter() {
        h.push(x.clone())
    }
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();
    while let Some(n) = h.pop() {
        if n.is_none() {
            continue;
        }
        let l = n?;
        let next = ListNode::new(l.val);
        let v = current?;
        v.next = Some(Box::new(next));
        current = v.next.as_mut();
        h.push(l.next)
    }
    head?.next
}
// linked_list divide_and_conquer heap
#[test]
fn test2_23() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 5, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode { val: 6, next: None })),
    }));
    let res = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 6, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(merge_k_lists(vec![l1, l2, l3]), res);
}
