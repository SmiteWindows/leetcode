// https://leetcode-cn.com/problems/rotate-list/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::mem::replace;
pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut current = &mut head;
    let mut len = 0;
    while let Some(a) = current {
        current = &mut a.next;
        len += 1;
    }
    k %= len;
    if k == 0 {
        head
    } else {
        current = &mut head;
        for _ in 0..len - k {
            current = &mut current.as_mut()?.next;
        }
        let mut new_head = current.take();
        let mut new_tail = &mut new_head;
        while let Some(b) = new_tail {
            new_tail = &mut b.next;
        }
        let _ = replace(new_tail, head); // 衔接
        new_head
    }
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
fn test1_61() {
    use leetcode_prelude::list;
    assert_eq!(rotate_right(list![1, 2, 3, 4, 5], 2), list![4, 5, 1, 2, 3]);
    assert_eq!(rotate_right(list![0, 1, 2], 4), list![2, 0, 1]);
}
