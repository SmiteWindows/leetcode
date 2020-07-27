// https://leetcode.com/problems/insertion-sort-list/
// Runtime: 24 ms
// Memory Usage: 2.5 MB
pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cursor = head;
    let mut guard = ListNode::new(0);
    while let Some(mut target) = cursor {
        cursor = target.next.take();
        let mut current = &mut guard;
        while current.next.is_some() && current.next.as_deref()?.val < target.val {
            current = current.next.as_deref_mut()?;
        }
        target.next = current.next.take();
        current.next = Some(target);
    }
    guard.next
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
    use leetcode_prelude::list;
    assert_eq!(insertion_sort_list(list![4, 2, 1, 3]), list![1, 2, 3, 4]);
    assert_eq!(
        insertion_sort_list(list![-1, 5, 3, 4, 0]),
        list![-1, 0, 3, 4, 5]
    );
    // let l1 = Some(Box::new(ListNode {
    //     val: 4,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 1,
    //             next: Some(Box::new(ListNode { val: 3, next: None })),
    //         })),
    //     })),
    // })); // 4->2->1->3
    // let l2 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode { val: 4, next: None })),
    //         })),
    //     })),
    // })); // 1->2->3->4
    // assert_eq!(l2, insertion_sort_list(l1));
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
    // })); // -1->5->3->4->0
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
    // })); // -1->0->3->4->5
    // assert_eq!(l4, insertion_sort_list(l3));
}
