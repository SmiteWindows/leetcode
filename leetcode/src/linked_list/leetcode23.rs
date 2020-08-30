// https://leetcode-cn.com/problems/merge-k-sorted-lists/
// Runtime: 208 ms
// Memory Usage: 3.2 MB
use std::cmp::Ordering;
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    let mut head = ListNode::new(0);
    let mut tail = &mut head;
    if lists.is_empty() {
        return None;
    }
    let len = lists.len();
    loop {
        let mut min = 0;
        for i in 0..len {
            if let Some(l) = lists[i].as_deref() {
                match lists[min].as_deref() {
                    Some(m) => {
                        if l.val <= m.val {
                            min = i;
                        }
                    }
                    _ => min = i,
                }
            }
        }
        match lists[min].take() {
            Some(m) => {
                tail.next = Some(Box::new(ListNode::new(m.val)));
                tail = tail.next.as_deref_mut()?;
                lists[min] = m.next; //note!
            }
            None => break,
        }
    }
    head.next
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

impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

// linked_list divide_and_conquer heap
#[test]
fn test1_23() {
    use leetcode_prelude::list;
    assert_eq!(
        merge_k_lists(vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]]),
        list![1, 1, 2, 3, 4, 4, 5, 6]
    );
}
