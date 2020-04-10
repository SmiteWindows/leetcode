// https://leetcode.com/problems/convert-bst-to-greater-tree/
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

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}
// tree
#[test]
fn test1_538() {
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 18,
        left: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
    })));
    assert_eq!(res, largest_values(root));
}
