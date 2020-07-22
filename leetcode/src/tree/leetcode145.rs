// https://leetcode.com/problems/binary-tree-postorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    postorder(root.as_deref(), &mut res);
    res
}

fn postorder(root: Option<&RefCell<TreeNode>>, nodes: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        postorder(node.left.as_deref(), nodes);
        postorder(node.right.as_deref(), nodes);
        nodes.push(node.val);
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
