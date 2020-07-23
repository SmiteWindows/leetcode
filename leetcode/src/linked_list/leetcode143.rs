// https://leetcode.com/problems/reorder-list/
// Runtime: 8 ms
// Memory Usage: 4.1 MB
use std::collections::VecDeque;
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut p = head.take();
    let mut deque = VecDeque::new();
    while let Some(mut n) = p {
        p = n.next.take();
        deque.push_back(Some(n));
    }
    let mut stack = vec![];
    let mut direction = true;
    while !deque.is_empty() {
        if direction {
            stack.push(deque.pop_front().unwrap());
        } else {
            stack.push(deque.pop_back().unwrap());
        }
        direction = !direction;
    }
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(last) = stack.pop() {
        let mut node = last.unwrap();
        node.next = prev;
        prev = Some(node);
    }
    *head = prev;
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
// linked_list
#[test]
fn test1_143() {
    let mut l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        })),
    }));
    let mut l3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
    }));
    reorder_list(&mut l1);
    assert_eq!(l2, l1);
    reorder_list(&mut l3);
    assert_eq!(l4, l3);
}
