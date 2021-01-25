// https://leetcode-cn.com/problems/merge-k-sorted-lists/
// Runtime: 4 ms
// Memory Usage: 3.3 MB
pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }
    let len = lists.len();
    let mut i = 1;
    while i < len {
        for x in (0..len - i).step_by(i * 2) {
            lists[x] = merge_two_lists(lists[x].take(), lists[x + i].take());
        }
        i *= 2;
    }
    lists[0].take()
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (t, None) | (None, t) => t,
        (Some(mut p1), Some(mut p2)) => {
            if p1.val < p2.val {
                p1.next = merge_two_lists(p1.next.take(), Some(p2));
                Some(p1)
            } else {
                p2.next = merge_two_lists(Some(p1), p2.next.take());
                Some(p2)
            }
        }
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
// linked_list divide_and_conquer heap
#[test]
fn test3_23() {
    use leetcode_prelude::list;
    assert_eq!(
        merge_k_lists(vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]]),
        list![1, 1, 2, 3, 4, 4, 5, 6]
    );
}
