// https://leetcode.com/problems/remove-linked-list-elements/
// Runtime: 4 ms
// Memory Usage: 2.7 MB
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut p = &mut head;
    while p.is_some() {
        if p.as_ref()?.val == val {
            *p = p.take()?.next.take();
        } else {
            p = &mut p.as_mut()?.next;
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
fn test1_203() {
    use leetcode_prelude::list;
    assert_eq!(
        remove_elements(list![1, 2, 6, 3, 4, 5, 6], 6),
        list![1, 2, 3, 4, 5]
    );
}
// let mut sentinel = Some(Box::new(ListNode { val: 0, next: head }));
// let mut p = &mut sentinel;
// while let Some(node) = &mut p.as_mut()?.next {
//     if node.val == val {
//         p.as_mut()?.next = node.next.take();
//     } else {
//         p = &mut p.as_mut()?.next;
//     }
// }
// sentinel?.next
