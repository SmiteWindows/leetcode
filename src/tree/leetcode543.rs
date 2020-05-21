// https://leetcode.com/problems/diameter-of-binary-tree/
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
// Runtime: 0 ms
// Memory Usage: 2.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut depth = 1;
    walk(root.as_ref(), &mut depth);
    depth - 1
}

fn walk(root: Option<&Rc<RefCell<TreeNode>>>, depth: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_ref(), depth);
        let right = walk(node.right.as_ref(), depth);
        *depth = (left + right + 1).max(*depth);
        left.max(right) + 1
    } else {
        0
    }
}
// tree
#[test]
fn test1_543() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(3, diameter_of_binary_tree(root));
}
