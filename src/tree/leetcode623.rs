// https://leetcode.com/problems/add-one-row-to-tree/
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

pub fn add_one_row(
    root: Option<Rc<RefCell<TreeNode>>>,
    v: i32,
    d: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}
