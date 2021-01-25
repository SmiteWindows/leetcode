// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut len = 0;
    let mut p = dummy_head.as_deref();
    while p?.next.is_some() {
        len += 1;
        p = p?.next.as_deref();
    }
    let idx = len - n;
    let mut p = dummy_head.as_deref_mut();
    for _ in 0..idx {
        p = p?.next.as_deref_mut();
    }
    let next = p.as_deref_mut()?.next.as_deref_mut()?.next.take();
    p.as_deref_mut()?.next = next;
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
// linked_list two_pointers
#[test]
fn test1_19() {
    use leetcode_prelude::list;
    assert_eq!(
        remove_nth_from_end(list![1, 2, 3, 4, 5], 2),
        list![1, 2, 3, 5]
    );
}
