// https://leetcode.com/problems/minimum-depth-of-binary-tree/
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
use std::{cell::RefCell, cmp::min, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.8 MB
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut min_depth = std::i32::MAX;
            if node.borrow().left.as_ref().is_none() && node.borrow().right.as_ref().is_none() {
                return 1;
            }
            if node.borrow().left.as_ref().is_some() {
                min_depth = min(helper(node.borrow().left.as_ref()), min_depth);
            }
            if node.borrow().right.as_ref().is_some() {
                min_depth = min(helper(node.borrow().right.as_ref()), min_depth);
            }
            min_depth + 1
        } else {
            0
        }
    }
    helper(root.as_ref())
}
// tree depth_first_search breadth_first_search
#[test]
fn test1_111() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(2, min_depth(root));
}
