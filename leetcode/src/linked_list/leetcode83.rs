// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut p1 = head.as_deref_mut()?;
    while let Some(p2) = p1.next.as_deref_mut() {
        if p1.val == p2.val {
            p1.next = p2.next.take();
        } else {
            p1 = p1.next.as_deref_mut()?;
        }
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
fn test1_83() {
    use leetcode_prelude::list;
    assert_eq!(delete_duplicates(list![1, 1, 2]), list![1, 2]);
    assert_eq!(delete_duplicates(list![1, 1, 2, 3, 3]), list![1, 2, 3]);
    // let l1 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 1,
    //         next: Some(Box::new(ListNode {
    //             val: 2,
    //             next: Some(Box::new(ListNode {
    //                 val: 3,
    //                 next: Some(Box::new(ListNode { val: 3, next: None })),
    //             })),
    //         })),
    //     })),
    // }));
    // let l2 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode { val: 3, next: None })),
    //     })),
    // }));
    // let l3 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 1,
    //         next: Some(Box::new(ListNode { val: 2, next: None })),
    //     })),
    // }));
    // let l4 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode { val: 2, next: None })),
    // }));
    // assert_eq!(l2, delete_duplicates(l1));
    // assert_eq!(l4, delete_duplicates(l3));
}
