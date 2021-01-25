// https://leetcode-cn.com/problems/insertion-sort-list/
// Runtime: 24 ms
// Memory Usage: 2.5 MB
pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cursor = head;
    let mut guard = ListNode::new(0);
    while let Some(mut target) = cursor {
        cursor = target.next.take();
        let mut current = &mut guard;
        while current.next.is_some() && current.next.as_deref()?.val < target.val {
            current = current.next.as_deref_mut()?;
        }
        target.next = current.next.take();
        current.next = Some(target);
    }
    guard.next
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
fn test1_147() {
    use leetcode_prelude::list;
    assert_eq!(insertion_sort_list(list![4, 2, 1, 3]), list![1, 2, 3, 4]);
    assert_eq!(
        insertion_sort_list(list![-1, 5, 3, 4, 0]),
        list![-1, 0, 3, 4, 5]
    );
}
