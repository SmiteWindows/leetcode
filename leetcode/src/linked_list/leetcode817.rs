// https://leetcode-cn.com/problems/linked-list-components/
// Runtime: 4 ms
// Memory Usage: 2.5 MB
use std::{collections::HashSet, iter::FromIterator};
pub fn num_components(head: Option<Box<ListNode>>, g: Vec<i32>) -> i32 {
    let mut p = head;
    let hs: HashSet<i32> = HashSet::from_iter(g);
    let mut open = false;
    let mut res = 0;
    while let Some(node) = p {
        if hs.contains(&node.val) {
            if !open {
                open = true;
            }
        } else if open {
            open = false;
            res += 1;
        }
        p = node.next;
    }
    if open {
        res += 1;
    }
    res
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
fn test1_817() {
    use leetcode_prelude::list;
    assert_eq!(num_components(list![0, 1, 2, 3], vec![0, 1, 3]), 2);
    assert_eq!(num_components(list![0, 1, 2, 3, 4], vec![0, 3, 1, 4]), 2);
}
