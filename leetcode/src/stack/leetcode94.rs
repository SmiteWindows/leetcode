// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;
    while curr.is_some() || !stack.is_empty() {
        while let Some(node) = curr {
            let left = node.borrow_mut().left.take();
            stack.push(Some(node));
            curr = left;
        }
        let node = stack.pop().unwrap().unwrap();
        res.push(node.borrow_mut().val);
        curr = node.borrow_mut().right.take();
    }
    res
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
fn test3_94() {
    use leetcode_prelude::btree;
    assert_eq!(inorder_traversal(btree![1, null, 2, 3]), vec![1, 3, 2]);
    assert_eq!(inorder_traversal(btree![]), vec![]);
    assert_eq!(inorder_traversal(btree![1]), vec![1]);
    assert_eq!(inorder_traversal(btree![1, 2]), vec![2, 1]);
    assert_eq!(inorder_traversal(btree![1, null, 2]), vec![1, 2]);
}
