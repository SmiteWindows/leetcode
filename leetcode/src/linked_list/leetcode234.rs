// https://leetcode-cn.com/problems/palindrome-linked-list/
// Runtime: 4 ms
// Memory Usage: 4.3 MB
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let mut dummy = head.as_deref();
    let mut len: usize = 0;
    while let Some(x) = dummy {
        len += 1;
        dummy = x.next.as_deref();
    }
    if len <= 1 {
        return true;
    }
    let mut first = &mut head;
    let mut second;
    let mid = len / 2;
    for _ in 1..mid {
        first = &mut first.as_mut().unwrap().next;
    }
    if len % 2 == 0 {
        second = first.as_mut().unwrap().next.take();
    } else {
        second = first.as_mut().unwrap().next.take().unwrap().next.take();
    }
    let mut prev = None;
    let mut cur = head;
    let mut next;
    while let Some(mut x) = cur {
        next = x.next.take();
        x.next = prev;
        prev = Some(x);
        cur = next;
    }
    while let (Some(node1), Some(node2)) = (prev, second) {
        if node1.val != node2.val {
            return false;
        } else {
            prev = node1.next;
            second = node2.next;
        }
    }
    true
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
fn test1_234() {
    use leetcode_prelude::list;
    assert_eq!(is_palindrome(list![1, 2]), false);
    assert_eq!(is_palindrome(list![1, 2, 2, 1]), true);
}
