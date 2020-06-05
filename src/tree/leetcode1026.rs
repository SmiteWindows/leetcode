// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/

use std::{cell::RefCell, rc::Rc};
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
// tree depth_first_search
#[test]
#[ignore]
fn test1_1026() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 14,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: None,
            }))),
        }))),
    })));
    assert_eq!(max_ancestor_diff(root), 7);
}
