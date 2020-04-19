// https://leetcode.com/problems/rotate-list/
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

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!()
    // let mut head = head;
    // let mut k = k;
    // if head.is_none() {
    //     return head;
    // }
    // let mut current = &mut head;
    // let mut len = 0;
    // while let Some(a) = current {
    //     current = &mut a.next;
    //     len += 1;
    // }
    // k %= len;
    // if k == 0 {
    //     head
    // } else {
    //     current = &mut head;
    //     for _ in 0..len - k {
    //         current = &mut current.as_mut()?.next;
    //     }
    //     let mut new_head = std::mem::replace(current, None);
    //     let mut new_tail = &mut new_head;
    //     while let Some(b) = new_tail {
    //         new_tail = &mut b.next;
    //     }
    //     std::mem::replace(new_tail, head); // 衔接
    //     new_head
    // }
}
// linked_list two_pointers
#[test]
#[ignore]
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
