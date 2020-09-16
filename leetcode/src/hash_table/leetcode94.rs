// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/

use std::{cell::RefCell, rc::Rc};
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    todo!()
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
// tree hash_table stack
#[test]
#[ignore]
fn test2_94() {
    use leetcode_prelude::btree;
    assert_eq!(inorder_traversal(btree![1, null, 2, 3]), vec![1, 3, 2]);
    assert_eq!(inorder_traversal(btree![]), vec![]);
    assert_eq!(inorder_traversal(btree![1]), vec![1]);
    assert_eq!(inorder_traversal(btree![1, 2]), vec![2, 1]);
    assert_eq!(inorder_traversal(btree![1, null, 2]), vec![1, 2]);
}
