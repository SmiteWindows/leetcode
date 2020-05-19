// https://leetcode.com/problems/binary-tree-tilt/
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
// Memory Usage: 2.6 MB
use std::{cell::RefCell, rc::Rc};
pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, tilt: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = walk(node.left.as_ref(), tilt);
            let right = walk(node.right.as_ref(), tilt);
            *tilt += (left - right).abs();
            left + right + node.val
        } else {
            0
        }
    }

    let mut tilt = 0;
    walk(root.as_ref(), &mut tilt);
    tilt
}
// tree
#[test]
fn test1_563() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(1, find_tilt(root));
}
