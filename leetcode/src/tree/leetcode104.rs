// https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_height(root.as_deref())
}

fn max_height(root: Option<&RefCell<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left_height = max_height(node.left.as_deref());
        let right_height = max_height(node.right.as_deref());
        left_height.max(right_height) + 1
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
fn test1_104() {
    use leetcode_prelude::btree;
    assert_eq!(3, max_depth(btree![3, 9, 20, null, null, 15, 7]));
}
