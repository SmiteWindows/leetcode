// https://leetcode.com/problems/trim-a-binary-search-tree/
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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::{cell::RefCell, rc::Rc};

pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    l: i32,
    r: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}