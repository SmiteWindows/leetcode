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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, flag: bool) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let leave = if flag && node.left.is_none() && node.right.is_none() {
                node.val
            } else {
                0
            };
            let left = helper(node.left.as_ref(), true);
            let right = helper(node.right.as_ref(), false);
            left + right + leave
        } else {
            0
        }
    }

    helper(root.as_ref(), false)
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
