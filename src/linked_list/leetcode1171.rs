// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// Runtime: 4 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut stack = vec![];
    let mut hm = HashMap::new();
    let mut sum = 0;
    hm.insert(0, 0);
    while let Some(node) = head {
        let val = node.val;
        head = node.next;
        if val == 0 {
            continue;
        }
        sum += val;
        if let Some(&size) = hm.get(&sum) {
            sum -= val;
            while stack.len() > size {
                let top = stack.pop().expect("exist");
                hm.remove(&sum);
                sum -= top;
            }
        } else {
            stack.push(val);
            hm.insert(sum, stack.len());
        }
    }
    let mut prev = None;
    while let Some(top) = stack.pop() {
        prev = Some(Box::new(ListNode {
            val: top,
            next: prev,
        }));
    }
    prev
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
        val: 3,
        next: Some(Box::new(ListNode { val: 1, next: None })),
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
