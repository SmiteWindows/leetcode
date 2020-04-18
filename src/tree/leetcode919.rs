// https://leetcode.com/problems/complete-binary-tree-inserter/
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

struct CBTInserter {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(v);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        todo!()
    }

    fn insert(&self, v: i32) -> i32 {
        todo!()
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}
