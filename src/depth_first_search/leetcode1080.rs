// https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths/
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

pub fn sufficient_subset(
    root: Option<Rc<RefCell<TreeNode>>>,
    limit: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}
