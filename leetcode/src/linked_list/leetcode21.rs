// https://leetcode.com/problems/merge-two-sorted-lists/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (t, None) | (None, t) => t,
        (Some(mut p1), Some(mut p2)) => {
            if p1.val < p2.val {
                p1.next = merge_two_lists(p1.next, Some(p2));
                Some(p1)
            } else {
                p2.next = merge_two_lists(Some(p1), p2.next);
                Some(p2)
            }
        }
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
// linked_list
#[test]
fn test1_21() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
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
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(merge_two_lists(l1, l2), res);
}
