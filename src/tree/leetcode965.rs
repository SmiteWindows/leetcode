// https://leetcode.com/problems/univalued-binary-tree/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_unival(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let left_unival = match node.left.as_ref() {
                    Some(left_node) => {
                        let left_node = left_node.borrow();
                        (node.val == left_node.val) && is_unival(node.left.as_ref())
                    }
                    None => true,
                };
                let right_unival = match node.right.as_ref() {
                    Some(right_node) => {
                        let right_node = right_node.borrow();
                        (node.val == right_node.val) && is_unival(node.right.as_ref())
                    }
                    None => true,
                };
                left_unival && right_unival
            }
            _ => true,
        }
    }
    is_unival(root.as_ref())
}
// tree
#[test]
fn test1_965() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(is_unival_tree(t1), true);
    assert_eq!(is_unival_tree(t2), false);
}
