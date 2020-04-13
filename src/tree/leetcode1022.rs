// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, mut sum: i32, val: i32) -> i32 {
        if let Some(node) = root {
            let new_val = val << 1 | node.as_ref().borrow().val;
            if node.as_ref().borrow().left.is_none() && node.as_ref().borrow().right.is_none() {
                sum += new_val;
            } else {
                sum = helper(node.as_ref().borrow().left.as_ref(), sum, new_val);
                sum = helper(node.as_ref().borrow().right.as_ref(), sum, new_val);
            }
        }
        sum
    }
    helper(root.as_ref(), 0, 0)
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
    })));
    assert_eq!(sum_root_to_leaf(root), 22);
}
