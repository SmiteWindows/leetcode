// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::cmp::Ordering::*;
use std::{cell::RefCell, rc::Rc};
pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(root.as_ref()).1
}

fn postorder(root: Option<&Rc<RefCell<TreeNode>>>) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let n = node.borrow();
        let (left_depth, left_tree) = postorder(n.left.as_ref());
        let (right_depth, rigth_tree) = postorder(n.right.as_ref());
        match left_depth.cmp(&right_depth) {
            Equal => (left_depth + 1, Some(node.clone())),
            Less => (right_depth + 1, rigth_tree),
            Greater => (left_depth + 1, left_tree),
        }
    } else {
        (0, None)
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
fn test1_865() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        }))),
    })));
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));
    assert_eq!(subtree_with_all_deepest(root), res);
}
