// https://leetcode.com/problems/maximum-depth-of-binary-tree/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
use std::{cell::RefCell, cmp::max, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_height(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node)=root{
            let left_height = max_height(node.as_ref().borrow().left.as_ref());
            let right_height = max_height(node.as_ref().borrow().right.as_ref());
            max(left_height, right_height) + 1
        } else{
            0
        }
    }
    max_height(root.as_ref())
}
// tree depth_first_search
#[test]
fn test1_104() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(3, max_depth(root));
}
