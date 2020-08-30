// https://leetcode-cn.com/problems/binary-tree-preorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    preorder(root.as_deref(), &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        res.push(node.val);
        preorder(node.left.as_deref(), res);
        preorder(node.right.as_deref(), res);
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
fn test1_144() {
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
