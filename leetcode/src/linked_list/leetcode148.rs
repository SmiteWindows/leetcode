// https://leetcode.com/problems/sort-list/
// Runtime: 4 ms
// Memory Usage: 4 MB
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut v = vec![];
    while let Some(node) = cur {
        v.push(node.val);
        cur = node.next;
    }
    v.sort_unstable();
    let mut prev = None;
    while let Some(last) = v.pop() {
        prev = Some(Box::new(ListNode {
            val: last,
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
// linked_list sort
#[test]
fn test1_148() {
    use leetcode_prelude::list;
    assert_eq!(sort_list(list![4, 2, 1, 3]), list![1, 2, 3, 4]);
    assert_eq!(sort_list(list![-1, 5, 3, 4, 0]), list![-1, 0, 3, 4, 5]);
    // let l1 = Some(Box::new(ListNode {
    //     val: 4,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 1,
    //             next: Some(Box::new(ListNode { val: 3, next: None })),
    //         })),
    //     })),
    // }));
    // let l2 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode { val: 4, next: None })),
    //         })),
    //     })),
    // }));
    // let l3 = Some(Box::new(ListNode {
    //     val: -1,
    //     next: Some(Box::new(ListNode {
    //         val: 5,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode {
    //                 val: 4,
    //                 next: Some(Box::new(ListNode { val: 0, next: None })),
    //             })),
    //         })),
    //     })),
    // }));
    // let l4 = Some(Box::new(ListNode {
    //     val: -1,
    //     next: Some(Box::new(ListNode {
    //         val: 0,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode {
    //                 val: 4,
    //                 next: Some(Box::new(ListNode { val: 5, next: None })),
    //             })),
    //         })),
    //     })),
    // }));
    // assert_eq!(l2, sort_list(l1));
    // assert_eq!(l4, sort_list(l3));
}
