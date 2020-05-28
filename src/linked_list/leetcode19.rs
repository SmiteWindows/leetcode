// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
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
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut len = 0;
    let mut p = dummy_head.as_deref();
    while p?.next.is_some() {
        len += 1;
        p = p?.next.as_deref();
    }
    let idx = len - n;
    let mut p = dummy_head.as_deref_mut();
    for _ in 0..idx {
        p = p?.next.as_deref_mut();
    }
    let next = p.as_deref_mut()?.next.as_deref_mut()?.next.take();
    p.as_deref_mut()?.next = next;
    dummy_head?.next
}
// linked_list two_pointers
#[test]
fn test1_19() {
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
    let res = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
    }));
    assert_eq!(res, remove_nth_from_end(l1, 2));
}
