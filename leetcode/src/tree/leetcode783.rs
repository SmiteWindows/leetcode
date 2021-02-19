// https://leetcode-cn.com/problems/minimum-distance-between-bst-nodes/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut prev = None;
    let mut min = i32::MAX;
    inorder(root.as_deref(), &mut prev, &mut min);
    min
}

fn inorder(root: Option<&RefCell<TreeNode>>, prev: &mut Option<i32>, min: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder(node.left.as_deref(), prev, min);
        let val = node.val;
        if let Some(prev_val) = prev.as_mut() {
            *min = (*min).min(val - *prev_val);
            *prev_val = val;
        } else {
            *prev = Some(val);
        }
        inorder(node.right.as_deref(), prev, min);
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
// tree recursion
#[test]
fn test1_783() {
    use leetcode_prelude::btree;
    assert_eq!(min_diff_in_bst(btree![4, 2, 6, 1, 3, null, null]), 1);
}
