// https://leetcode.com/problems/linked-list-components/
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
// Runtime: 8 ms
// Memory Usage: 2.8 MB
use std::collections::HashSet;
pub fn num_components(head: Option<Box<ListNode>>, g: Vec<i32>) -> i32 {
    let mut gset = HashSet::new();
    let mut res = 0;
    for x in g {
        gset.insert(x);
    }
    let mut cur = head.as_deref();
    while let Some(x) = cur {
        if gset.contains(&x.val)
            && (x.next.is_none() || !gset.contains(&x.next.as_deref().unwrap().val))
        {
            res += 1;
        }
        cur = x.next.as_deref();
    }
    res
}
// linked_list
#[test]
fn test1_817() {
    let l1 = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        })),
    }));
    assert_eq!(num_components(l3, vec![4]), 1);
    assert_eq!(num_components(l1, vec![0, 1, 3]), 2);
    assert_eq!(num_components(l2, vec![0, 3, 1, 4]), 2);
}
