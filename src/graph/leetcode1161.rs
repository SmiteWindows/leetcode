// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/

use std::{cell::RefCell, rc::Rc};
pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!()
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
// graph
#[test]
#[ignore]
fn test1_1161() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(max_level_sum(root), 2);
}
