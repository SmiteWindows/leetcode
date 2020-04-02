// https://leetcode.com/problems/range-sum-of-bst/
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

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    todo!()
}
