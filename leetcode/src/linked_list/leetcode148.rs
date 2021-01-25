// https://leetcode-cn.com/problems/sort-list/
// Runtime: 4 ms
// Memory Usage: 4 MB
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut v = vec![];
    while let Some(node) = cur {
        v.push(node.val);
        cur = node.next;
    }
    v.sort_unstable();
    let mut prev = None;
    while let Some(last) = v.pop() {
        prev = Some(Box::new(ListNode {
            val: last,
            next: prev,
        }));
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
// linked_list sort
#[test]
fn test1_148() {
    use leetcode_prelude::list;
    assert_eq!(sort_list(list![4, 2, 1, 3]), list![1, 2, 3, 4]);
    assert_eq!(sort_list(list![-1, 5, 3, 4, 0]), list![-1, 0, 3, 4, 5]);
}
