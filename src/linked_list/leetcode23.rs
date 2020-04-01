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
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}
use std::cmp::Ordering;

/// Runtime: 208 ms
/// Memory Usage: 3.2 MB
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    if lists.is_empty() {
        return None;
    }
    let len = lists.len();
    loop {
        let mut min = 0;
        for i in 0..len {
            if let Some(l) = lists[i].as_ref() {
                match lists[min].as_ref() {
                    Some(m) => {
                        if l.val <= m.val {
                            min = i;
                        }
                    }
                    _ => min = i,
                }
            }
        }

        match lists[min].take() {
            Some(m) => {
                tail.next = Some(Box::new(ListNode::new(m.val)));
                tail = tail.next.as_mut()?;
                lists[min] = m.next; //note!
            }
            None => break,
        }
    }

    head.next
}
#[test]
fn test1_23() {
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
        next: Some(Box::new(ListNode { val: 5, next: None })),
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
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(merge_k_lists(vec![l1, l2, l3]), res);
}