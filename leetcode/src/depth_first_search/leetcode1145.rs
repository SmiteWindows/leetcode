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
fn test2_1145() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(btree_game_winning_move(root, 11, 3), true);
}
