// https://leetcode.com/problems/swap-nodes-in-pairs/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut head = dummy_head.as_deref_mut();
    loop {
        let mut a = head.as_deref_mut()?.next.take();
        if a.is_none() {
            break;
        }
        let mut b = a.as_deref_mut()?.next.take();
        if b.is_none() {
            head.as_deref_mut()?.next = a;
            break;
        }
        let next = b.as_deref_mut()?.next.take();

        a.as_deref_mut()?.next = next;
        b.as_deref_mut()?.next = a;
        head.as_deref_mut()?.next = b;
        head = head?.next.as_deref_mut()?.next.as_deref_mut();
    }
    dummy_head?.next
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
fn test1_24() {
    use leetcode_prelude::list;
    assert_eq!(swap_pairs(list![1, 2, 3, 4]), list![2, 1, 4, 3]);
    // let l1 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode { val: 4, next: None })),
    //         })),
    //     })),
    // }));
    // let l2 = Some(Box::new(ListNode {
    //     val: 2,
    //     next: Some(Box::new(ListNode {
    //         val: 1,
    //         next: Some(Box::new(ListNode {
    //             val: 4,
    //             next: Some(Box::new(ListNode { val: 3, next: None })),
    //         })),
    //     })),
    // }));
    // assert_eq!(l2, swap_pairs(l1));
}
