// https://leetcode.com/problems/partition-list/
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
/// Runtime: 4 ms
/// Memory Usage: 2 MB
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut lower = Some(Box::new(ListNode::new(0)));
    let mut higher = Some(Box::new(ListNode::new(0)));
    let mut lower_tail = lower.as_mut();
    let mut higher_tail = higher.as_mut();
    let mut head = head;
    while let Some(mut inner) = head {
        let next = inner.next.take();
        if inner.val < x {
            lower_tail.as_mut()?.next = Some(inner);
            lower_tail = lower_tail?.next.as_mut();
        } else {
            higher_tail.as_mut()?.next = Some(inner);
            higher_tail = higher_tail?.next.as_mut();
        }
        head = next
    }
    lower_tail.as_mut()?.next = higher?.next.take();
    lower?.next.take()
}
#[test]
fn test1_86() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 2, next: None })),
                    })),
                })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(l2, partition(l1, 3));
}