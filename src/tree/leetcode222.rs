//https://leetcode.com/problems/count-complete-tree-nodes/
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
// Runtime: 4 ms
// Memory Usage: 4.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    walk(root.as_ref())
}

fn walk(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_ref());
        let right = walk(node.right.as_ref());
        1 + left + right
    } else {
        0
    }
}
// tree binary_search
#[test]
fn test1_222() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    })));
    assert_eq!(6, count_nodes(root));
}
