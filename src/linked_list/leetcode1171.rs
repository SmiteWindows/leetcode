// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
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
// use std::collections::HashMap;
pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}
// linked_list
#[test]
#[ignore]
fn test1_1171() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: -3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: -3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode { val: 1, next: None })),
    }));
    let l5 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        })),
    }));
    let l6 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let l7 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode {
                        val: -2,
                        next: None,
                    })),
                })),
            })),
        })),
    }));
    let l8 = Some(Box::new(ListNode { val: 1, next: None }));
    assert_eq!(remove_zero_sum_sublists(l1), l2);
    assert_eq!(remove_zero_sum_sublists(l3), l4);
    assert_eq!(remove_zero_sum_sublists(l5), l6);
    assert_eq!(remove_zero_sum_sublists(l7), l8);
}
