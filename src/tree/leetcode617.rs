// https://leetcode.com/problems/merge-two-binary-trees/
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
// Memory Usage: 2.2 MB
pub fn merge_trees(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if t1.is_none() {
        return t2;
    }
    if t2.is_none() {
        return t1;
    }
    t1.as_ref().unwrap().borrow_mut().val += t2.as_ref().unwrap().borrow().val;
    let t1_left = t1.as_ref().unwrap().borrow().left.clone();
    let t2_left = t2.as_ref().unwrap().borrow().left.clone();
    let t1_right = t1.as_ref().unwrap().borrow().right.clone();
    let t2_right = t2.as_ref().unwrap().borrow().right.clone();
    t1.as_ref().unwrap().borrow_mut().left = merge_trees(t1_left, t2_left);
    t1.as_ref().unwrap().borrow_mut().right = merge_trees(t1_right, t2_right);
    t1
}
// tree
#[test]
fn test1_617() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(res, merge_trees(t1, t2));
}
