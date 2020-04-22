// https://leetcode.com/problems/convert-bst-to-greater-tree/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 4 ms
// Memory Usage: 2.9 MB
pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = root {
            walk(node.borrow().right.as_ref(), sum);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            walk(node.borrow().left.as_ref(), sum);
        }
    }

    let mut sum = 0;
    walk(root.as_ref(), &mut sum);
    root
}
// tree
#[test]
fn test1_538() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
    })));
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 18,
        left: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
    })));
    assert_eq!(res, convert_bst(root));
}
