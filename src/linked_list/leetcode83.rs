// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
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
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut head = head;
    let mut p1 = head.as_mut()?;
    while let Some(p2) = p1.next.as_mut() {
        if p1.val == p2.val {
            p1.next = p2.next.take();
        } else {
            p1 = p1.next.as_mut()?;
        }
    }
    head
}
#[test]
fn test1_83() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));
    assert_eq!(l2, delete_duplicates(l1));
    assert_eq!(l4, delete_duplicates(l3));
}
