// https://leetcode.com/problems/binary-tree-coloring-game/
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

pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
    todo!()
}
