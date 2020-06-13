// https://leetcode.com/problems/rotate-list/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut p = head.as_deref();
    let mut n = 0;
    while let Some(node) = p {
        p = node.next.as_deref();
        n += 1;
    }
    if n < 2 {
        return head;
    }
    let k = k as usize % n;
    if k == 0 {
        return head;
    }
    let mut i = 0;
    let mut p = head.as_deref_mut();
    let mut new_head = None;
    while let Some(node) = p {
        if i + k == n - 1 {
            new_head = node.next.take();
            break;
        } else {
            p = node.next.as_deref_mut();
        }
        i += 1;
    }
    let mut p = new_head.as_deref_mut();
    while let Some(node) = p {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        p = node.next.as_deref_mut();
    }
    new_head
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
// linked_list two_pointers
#[test]
fn test2_61() {
    let l1 = Some(Box::new(ListNode {
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
        val: 4,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));
    assert_eq!(l2, rotate_right(l1, 2));
    assert_eq!(l4, rotate_right(l3, 4));
}
