// https://leetcode-cn.com/problems/partition-list/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut lower = Some(Box::new(ListNode::new(0)));
    let mut higher = Some(Box::new(ListNode::new(0)));
    let mut lower_tail = lower.as_deref_mut();
    let mut higher_tail = higher.as_deref_mut();
    while let Some(mut inner) = head {
        let next = inner.next.take();
        if inner.val < x {
            lower_tail.as_deref_mut()?.next = Some(inner);
            lower_tail = lower_tail?.next.as_deref_mut();
        } else {
            higher_tail.as_deref_mut()?.next = Some(inner);
            higher_tail = higher_tail?.next.as_deref_mut();
        }
        head = next
    }
    lower_tail.as_deref_mut()?.next = higher?.next.take();
    lower?.next.take()
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
fn test1_86() {
    use leetcode_prelude::list;
    assert_eq!(
        partition(list![1, 4, 3, 2, 5, 2], 3),
        list![1, 2, 2, 4, 3, 5]
    );
}
