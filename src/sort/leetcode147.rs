// https://leetcode.com/problems/insertion-sort-list/

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        prev = insert(prev, Some(node));
    }
    prev
}

fn insert(prev: Option<Box<ListNode>>, mut link: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let val = link.as_deref()?.val;
    if let Some(mut node) = prev {
        if node.val > val {
            link.as_deref_mut()?.next = Some(node);
            link
        } else {
            node.next = insert(node.next.take(), link);
            Some(node)
        }
    } else {
        link
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
// linked_list sort
#[test]
fn test2_147() {
    let l1 = Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: -1,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 0, next: None })),
                })),
            })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: -1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    assert_eq!(l2, insertion_sort_list(l1));
    assert_eq!(l4, insertion_sort_list(l3));
}
