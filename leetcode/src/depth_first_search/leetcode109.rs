// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// Runtime: 4 ms
// Memory Usage: 3.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut ptr = dummy_head.as_deref()?;
    let mut length = 0;
    while ptr.next.is_some() {
        length += 1;
        ptr = ptr.next.as_deref()?;
    }
    helper(dummy_head.as_deref_mut()?.next.take(), length)
}

fn helper(mut head: Option<Box<ListNode>>, length: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if length == 0 {
        return None;
    }
    if length == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(head.as_deref()?.val))));
    }
    let mut ptr = head.as_deref_mut()?;
    for _ in 0..length / 2 - 1 {
        ptr = ptr.next.as_deref_mut()?;
    }
    let right_half = ptr.next.take();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        right_half.as_deref()?.val,
    ))));
    root.as_deref()?.borrow_mut().left = helper(head, length / 2);
    root.as_deref()?.borrow_mut().right = helper(right_half?.next, length - length / 2 - 1);
    root
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
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}
// linked_list depth_first_search
#[test]
fn test2_109() {
    use leetcode_prelude::{btree, list};
    assert_eq!(
        sorted_list_to_bst(list![-10, -3, 0, 5, 9]),
        btree![0, -3, 9, -10, null, 5]
    );
}
