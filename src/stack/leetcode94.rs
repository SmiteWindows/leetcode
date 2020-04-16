// https://leetcode.com/problems/binary-tree-inorder-traversal/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
// Memory Usage: 2 MB
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
// tree hash_table stack
#[test]
fn test3_94() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));
    assert_eq!(vec![1, 3, 2], inorder_traversal(root));
}
