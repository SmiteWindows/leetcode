// https://leetcode.com/problems/odd-even-linked-list/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}
