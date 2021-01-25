// https://leetcode-cn.com/problems/rotate-list/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut p = head.as_deref();
    let mut n = 0;
    while let Some(node) = p {
        p = node.next.as_deref();
        n += 1;
    }
    if n < 2 {
        return head;
    }
    let k = k as usize % n;
    if k == 0 {
        return head;
    }
    let mut i = 0;
    let mut p = head.as_deref_mut();
    let mut new_head = None;
    while let Some(node) = p {
        if i + k == n - 1 {
            new_head = node.next.take();
            break;
        } else {
            p = node.next.as_deref_mut();
        }
        i += 1;
    }
    let mut p = new_head.as_deref_mut();
    while let Some(node) = p {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        p = node.next.as_deref_mut();
    }
    new_head
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
fn test2_61() {
    use leetcode_prelude::list;
    assert_eq!(rotate_right(list![1, 2, 3, 4, 5], 2), list![4, 5, 1, 2, 3]);
    assert_eq!(rotate_right(list![0, 1, 2], 4), list![2, 0, 1]);
}
