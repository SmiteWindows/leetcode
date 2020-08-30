// https://leetcode-cn.com/problems/sum-of-root-to-leaf-binary-numbers/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), 0, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, parent_val: i32, sum: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = parent_val << 1 | node.val;
        if node.left.is_none() && node.right.is_none() {
            *sum += val;
        }
        preorder(node.left.as_deref(), val, sum);
        preorder(node.right.as_deref(), val, sum);
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
fn test1_1022() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    }))); // [1,0,1,0,1,0,1]
    assert_eq!(sum_root_to_leaf(root), 22);
}
