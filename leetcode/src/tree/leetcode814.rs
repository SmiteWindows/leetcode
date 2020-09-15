// https://leetcode-cn.com/problems/binary-tree-pruning/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(root)
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let val = node.borrow().val;
        let left = postorder(node.borrow_mut().left.take());
        let right = postorder(node.borrow_mut().right.take());
        if left.is_none() && right.is_none() && val == 0 {
            None
        } else {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    } else {
        None
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
fn test1_814() {
    use leetcode_prelude::btree;
    assert_eq!(
        prune_tree(btree![1, null, 0, 0, 1]),
        btree![1, null, 0, null, 1]
    );

    assert_eq!(
        prune_tree(btree![1, 0, 1, 0, 0, 0, 1]),
        btree![1, null, 1, null, 1]
    );

    assert_eq!(
        prune_tree(btree![1, 1, 0, 1, 1, 0, 1, 0]),
        btree![1, 1, 0, 1, 1, null, 1]
    );
}
