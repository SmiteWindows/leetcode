// https://leetcode.com/problems/palindrome-linked-list/
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
// Runtime: 4 ms
// Memory Usage: 4.3 MB
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let mut dummy = &head;
    let mut len: usize = 0;
    while dummy.is_some() {
        len += 1;
        dummy = &dummy.as_ref().unwrap().next;
    }
    if len <= 1 {
        return true;
    }
    let mut first = &mut head;
    let mut second: Option<Box<ListNode>>;
    let mid = len / 2;
    for _ in 1..mid {
        first = &mut first.as_mut().unwrap().next;
    }
    if len % 2 == 0 {
        second = first.as_mut().unwrap().next.take();
    } else {
        second = first.as_mut().unwrap().next.take().unwrap().next.take();
    }
    let mut prev: Option<Box<ListNode>> = None;
    let mut cur = head;
    let mut next: Option<Box<ListNode>>;
    while cur.is_some() {
        let mut node = cur.unwrap();
        next = node.next.take();
        node.next = prev;
        prev = Some(node);
        cur = next;
    }
    while prev.is_some() && second.is_some() {
        let node1 = prev.unwrap();
        let node2 = second.unwrap();
        if node1.val != node2.val {
            return false;
        } else {
            prev = node1.next;
            second = node2.next;
        }
    }
    true
}
#[test]
fn test1_234() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        })),
    }));
    assert_eq!(is_palindrome(l1), false);
    assert_eq!(is_palindrome(l2), true);
}
