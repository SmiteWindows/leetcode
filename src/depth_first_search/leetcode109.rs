// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
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
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 4 ms
// Memory Usage: 3.3 MB
pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(mut head: Option<Box<ListNode>>, length: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if length == 0 {
            return None;
        }
        if length == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(head.as_ref()?.val))));
        }
        let mut ptr = head.as_mut()?;
        for _ in 0..length / 2 - 1 {
            ptr = ptr.next.as_mut()?;
        }
        let right_half = ptr.next.take();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            right_half.as_ref()?.val,
        ))));
        root.as_ref()?.borrow_mut().left = helper(head, length / 2);
        root.as_ref()?.borrow_mut().right = helper(right_half?.next, length - length / 2 - 1);
        root
    }
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut ptr = dummy_head.as_ref()?;
    let mut length = 0;
    while ptr.next.is_some() {
        length += 1;
        ptr = ptr.next.as_ref()?;
    }
    helper(dummy_head.as_mut()?.next.take(), length)
}
// linked_list depth_first_search
#[test]
fn test2_109() {
    let head = Some(Box::new(ListNode {
        val: -10,
        next: Some(Box::new(ListNode {
            val: -3,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        })),
    }));
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
    })));
    assert_eq!(res, sorted_list_to_bst(head));
}
