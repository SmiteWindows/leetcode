// https://leetcode.com/problems/sum-of-left-leaves/
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
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    helper(root.as_deref(), false)
}

fn helper(root: Option<&RefCell<TreeNode>>, flag: bool) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let leave = if flag && node.left.is_none() && node.right.is_none() {
            node.val
        } else {
            0
        };
        let left = helper(node.left.as_deref(), true);
        let right = helper(node.right.as_deref(), false);
        left + right + leave
    } else {
        0
    }
}
// tree
#[test]
fn test1_404() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(24, sum_of_left_leaves(root));
}
