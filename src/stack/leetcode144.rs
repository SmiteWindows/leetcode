// https://leetcode.com/problems/binary-tree-preorder-traversal/
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
// Memory Usage: 2 MB
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack=Vec::new();
    let mut res=Vec::new();
    if root.is_none() {
        return res;
    }
    stack.push(root);
    while !stack.is_empty() {
        let node = stack.pop().unwrap().unwrap();
        let node = node.borrow();
        res.push(node.val);
        if node.right.is_some() {
            stack.push(node.right.clone());
        }
        if node.left.is_some() {
            stack.push(node.left.clone());
        }
    }
    res
}
// tree stack
#[test]
fn test2_144() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));
    let res = vec![1, 2, 3];
    assert_eq!(res, preorder_traversal(root));
}
