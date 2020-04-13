// https://leetcode.com/problems/remove-linked-list-elements/
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
// Runtime: 4 ms
// Memory Usage: 2.7 MB
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut p = &mut head;
    while p.is_some() {
        if p.as_ref()?.val == val {
            *p = p.take()?.next.take();
        } else {
            p = &mut p.as_mut()?.next;
        }
    }
    head
}
// linked_list
#[test]
fn test1_203() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 6,
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
        })),
    }));
    let l2 = Some(Box::new(ListNode {
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
    assert_eq!(remove_elements(l1, 6), l2);
}
// let mut sentinel = Some(Box::new(ListNode { val: 0, next: head }));
// let mut p = &mut sentinel;
// while let Some(node) = &mut p.as_mut()?.next {
//     if node.val == val {
//         p.as_mut()?.next = node.next.take();
//     } else {
//         p = &mut p.as_mut()?.next;
//     }
// }
// sentinel?.next
