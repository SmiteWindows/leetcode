// https://leetcode.com/problems/insertion-sort-list/
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
// Runtime: 36 ms
// Memory Usage: 2.5 MB
pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut head = head?;
    let mut curr = head.next.take();
    let mut res = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode::new(head.val))),
    }));
    while curr.is_some() {
        let mut prev = res.as_mut()?;
        let curr_val = curr.as_ref()?.val;
        while prev.next.is_some() {
            if prev.next.as_ref()?.val >= curr_val {
                break;
            }
            prev = prev.next.as_mut()?;
        }
        let mut data = ListNode::new(curr_val);
        data.next = prev.next.take();
        prev.next = Some(Box::new(data));
        curr = curr?.next.take();
    }
    res?.next
}
#[test]
fn test1_147() {
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
