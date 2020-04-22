// https://leetcode.com/problems/validate-binary-search-tree/
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
// Runtime: 0 ms
// Memory Usage: 2.9 MB
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(
        root: Option<&Rc<RefCell<TreeNode>>>,
        lower: Option<i32>,
        upper: Option<i32>,
    ) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            if lower.is_some() && val <= lower.unwrap() {
                return false;
            }
            if upper.is_some() && val >= upper.unwrap() {
                return false;
            }
            if !helper(node.left.as_ref(), lower, Some(val)) {
                return false;
            }
            if !helper(node.right.as_ref(), Some(val), upper) {
                return false;
            }
            true
        } else {
            true
        }
    }
    
    helper(root.as_ref(), None, None)
}
// tree depth_first_search
#[test]
fn test2_98() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    assert_eq!(true, is_valid_bst(t1));
    assert_eq!(false, is_valid_bst(t2));
}
