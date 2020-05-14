// https://leetcode.com/problems/binary-tree-coloring-game/
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
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        x: i32,
        left_count: &mut i32,
        right_count: &mut i32,
    ) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = walk(node.left.as_ref(), x, left_count, right_count);
            let right = walk(node.right.as_ref(), x, left_count, right_count);
            if node.val == x {
                *left_count = left;
                *right_count = right;
            }
            left + right + 1
        } else {
            0
        }
    }

    let (mut left_count, mut right_count) = (0, 0);
    walk(root.as_ref(), x, &mut left_count, &mut right_count);
    (n - left_count - right_count - 1)
        .max(right_count)
        .max(left_count)
        > n / 2
}
// tree depth_first_search
#[test]
fn test1_1145() {
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
