// https://leetcode.com/problems/middle-of-the-linked-list/
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
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut ptr = head.clone();
    while ptr.is_some() && ptr.as_deref()?.next.is_some() {
        ptr = ptr?.next?.next;
        head = head?.next;
    }
    head
}
// linked_list
#[test]
fn test1_876() {
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
        val: 3,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 5, next: None })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 6, next: None })),
                    })),
                })),
            })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode { val: 6, next: None })),
        })),
    }));
    assert_eq!(l2, middle_node(l1));
    assert_eq!(l4, middle_node(l3));
}
