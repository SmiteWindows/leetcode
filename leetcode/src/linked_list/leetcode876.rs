// https://leetcode.com/problems/middle-of-the-linked-list/
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
fn test1_876() {
    use leetcode_prelude::list;
    assert_eq!(middle_node(list![1, 2, 3, 4, 5]), list![3, 4, 5]);
    assert_eq!(middle_node(list![1, 2, 3, 4, 5, 6]), list![4, 5, 6]);
    // let l1 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode {
    //                 val: 4,
    //                 next: Some(Box::new(ListNode { val: 5, next: None })),
    //             })),
    //         })),
    //     })),
    // }));
    // let l2 = Some(Box::new(ListNode {
    //     val: 3,
    //     next: Some(Box::new(ListNode {
    //         val: 4,
    //         next: Some(Box::new(ListNode { val: 5, next: None })),
    //     })),
    // }));
    // let l3 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode {
    //                 val: 4,
    //                 next: Some(Box::new(ListNode {
    //                     val: 5,
    //                     next: Some(Box::new(ListNode { val: 6, next: None })),
    //                 })),
    //             })),
    //         })),
    //     })),
    // }));
    // let l4 = Some(Box::new(ListNode {
    //     val: 4,
    //     next: Some(Box::new(ListNode {
    //         val: 5,
    //         next: Some(Box::new(ListNode { val: 6, next: None })),
    //     })),
    // }));
    // assert_eq!(l2, middle_node(l1));
    // assert_eq!(l4, middle_node(l3));
}
