// https://leetcode.com/problems/next-greater-node-in-linked-list/
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

pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    todo!()
}
