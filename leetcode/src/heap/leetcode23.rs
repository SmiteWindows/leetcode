// https://leetcode-cn.com/problems/merge-k-sorted-lists/
// Runtime: 4 ms
// Memory Usage: 3 MB
use std::{cmp::Ordering, collections::BinaryHeap};
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut h = BinaryHeap::with_capacity(lists.len());
    for x in lists.iter() {
        h.push(x.clone())
    }
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_deref_mut();
    while let Some(n) = h.pop() {
        if n.is_none() {
            continue;
        }
        let l = n?;
        let next = ListNode::new(l.val);
        let v = current?;
        v.next = Some(Box::new(next));
        current = v.next.as_deref_mut();
        h.push(l.next)
    }
    head?.next
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
// linked_list divide_and_conquer heap
#[test]
fn test2_23() {
    use leetcode_prelude::list;
    assert_eq!(
        merge_k_lists(vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]]),
        list![1, 1, 2, 3, 4, 4, 5, 6]
    );
}
