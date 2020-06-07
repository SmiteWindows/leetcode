// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
// Runtime: 0 ms
// Memory Usage: 2.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), None, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, mut parent: Option<(i32, i32)>, abs: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        parent = if let Some((min, max)) = parent {
            *abs = (*abs).max((min - val).abs().max((max - val).abs()));
            Some((min.min(val), max.max(val)))
        } else {
            Some((val, val))
        };
        preorder(node.left.as_deref(), parent, abs);
        preorder(node.right.as_deref(), parent, abs);
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
// tree depth_first_search
#[test]
fn test1_1026() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 14,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: None,
            }))),
        }))),
    })));
    assert_eq!(max_ancestor_diff(root), 7);
}
