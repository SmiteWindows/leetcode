// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
/// 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
/// Runtime: 0 ms
/// Memory Usage: 2 MB
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut len = 0;
    let mut p = dummy_head.as_ref();
    while p?.next.is_some() {
        len += 1;
        p = p?.next.as_ref();
    }
    let idx = len - n;
    let mut p = dummy_head.as_mut();
    for _ in 0..idx {
        p = p?.next.as_mut();
    }
    let next = p.as_mut()?.next.as_mut()?.next.take();
    p.as_mut()?.next = next;
    dummy_head?.next
}
#[test]
fn test1_19(){
    let mut a = Some(Box::new(ListNode::new(1)));
    a.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    a.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    a.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    a.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
    let mut b = Some(Box::new(ListNode::new(1)));
    b.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    b.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    b.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
    assert_eq!(b,remove_nth_from_end(a, 2));
}