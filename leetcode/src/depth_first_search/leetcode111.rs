// https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    helper(root.as_deref())
}

fn helper(root: Option<&RefCell<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut min_depth = i32::MAX;
        if node.left.is_none() && node.right.is_none() {
            return 1;
        }
        if node.left.is_some() {
            min_depth = min_depth.min(helper(node.left.as_deref()));
        }
        if node.right.is_some() {
            min_depth = min_depth.min(helper(node.right.as_deref()));
        }
        min_depth + 1
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
// tree depth_first_search breadth_first_search
#[test]
fn test2_111() {
    use leetcode_prelude::btree;
    assert_eq!(min_depth(btree![3, 9, 20, null, null, 15, 7]), 2);
}
