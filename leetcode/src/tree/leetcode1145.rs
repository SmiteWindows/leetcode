// https://leetcode-cn.com/problems/binary-tree-coloring-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
    let mut left = 0;
    let mut right = 0;
    postorder(root.as_deref(), x, &mut left, &mut right);
    (n - left - right - 1).max(right).max(left) > n / 2
}

fn postorder(root: Option<&RefCell<TreeNode>>, x: i32, left: &mut i32, right: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let l = postorder(node.left.as_deref(), x, left, right);
        let r = postorder(node.right.as_deref(), x, left, right);
        if val == x {
            *left = l;
            *right = r;
        }
        l + r + 1
    } else {
        0
    }
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
fn test1_1145() {
    use leetcode_prelude::btree;
    assert_eq!(
        btree_game_winning_move(btree![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 11, 3),
        true
    );
}
