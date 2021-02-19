// https://leetcode-cn.com/problems/binary-tree-tilt/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
use std::{cell::RefCell, rc::Rc};
pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut tilt = 0;
    walk(root.as_deref(), &mut tilt);
    tilt
}

fn walk(root: Option<&RefCell<TreeNode>>, tilt: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_deref(), tilt);
        let right = walk(node.right.as_deref(), tilt);
        *tilt += (left - right).abs();
        left + right + node.val
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
// tree
#[test]
fn test1_563() {
    use leetcode_prelude::btree;
    assert_eq!(find_tilt(btree![1, 2, 3]), 1);
}
