// https://leetcode.com/problems/binary-tree-postorder-traversal/
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
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    stack.push(root);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        res.insert(0, node.as_ref().unwrap().borrow().val);
        if node.as_ref().unwrap().borrow().left.is_some() {
            stack.push(node.as_ref().unwrap().borrow().left.clone());
        }
        if node.as_ref().unwrap().borrow().right.is_some() {
            stack.push(node.as_ref().unwrap().borrow().right.clone());
        }
    }
    res
}
// tree stack
#[test]
fn test1_145() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));
    assert_eq!(vec![3, 2, 1], postorder_traversal(root));
}
