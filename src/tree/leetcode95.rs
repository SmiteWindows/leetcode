// https://leetcode.com/problems/unique-binary-search-trees-ii/
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
use std::{rc::Rc, cell::RefCell};

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    todo!()
}
