// https://leetcode-cn.com/problems/reverse-nodes-in-k-group/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::collections::VecDeque;
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut p = head;
    let mut count = 0;
    let mut queue = VecDeque::new();
    let k = k as usize;
    while let Some(mut node) = p {
        p = node.next.take();
        queue.push_back(Some(node));
        count += 1;
        if count == k {
            break;
        }
    }
    if queue.len() == k {
        let mut prev = reverse_k_group(p, k as i32);
        while let Some(link) = queue.pop_front() {
            if let Some(mut node) = link {
                node.next = prev;
                prev = Some(node);
            }
        }
        prev
    } else {
        let mut prev = None;
        while let Some(link) = queue.pop_back() {
            if let Some(mut node) = link {
                node.next = prev;
                prev = Some(node);
            }
        }
        prev
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
// linked_list
#[test]
fn test1_25() {
    use leetcode_prelude::list;
    assert_eq!(
        reverse_k_group(list![1, 2, 3, 4, 5], 2),
        list![2, 1, 4, 3, 5]
    );
    assert_eq!(
        reverse_k_group(list![1, 2, 3, 4, 5], 3),
        list![3, 2, 1, 4, 5]
    );
}
