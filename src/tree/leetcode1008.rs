// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
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

pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}
// tree
#[test]
fn test1_1008() {
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
        }))),
    })));
    assert_eq!(bst_from_preorder(vec![8, 5, 1, 7, 10, 12]), res);
}
