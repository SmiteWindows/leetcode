// https://leetcode-cn.com/problems/reverse-linked-list/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
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
fn test1_206() {
    use leetcode_prelude::list;
    assert_eq!(reverse_list(list![1, 2, 3, 4, 5]), list![5, 4, 3, 2, 1]);
}
