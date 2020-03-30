// https://leetcode.com/problems/insertion-sort-list/
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

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}
