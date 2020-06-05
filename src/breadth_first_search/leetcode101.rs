// https://leetcode.com/problems/symmetric-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = Vec::new();
    queue.push(root.clone());
    queue.push(root);
    while !queue.is_empty() {
        let t1 = queue.pop().expect("exist");
        let t2 = queue.pop().expect("exist");
        if t1.is_none() && t2.is_none() {
            continue;
        }
        if t1.is_none() || t2.is_none() {
            return false;
        }
        let (t1, t2) = (t1.expect("exist"), t2.expect("exist"));
        let (t1, t2) = (t1.borrow(), t2.borrow());
        if t1.val != t2.val {
            return false;
        }
        queue.push(t1.left.clone());
        queue.push(t2.right.clone());
        queue.push(t1.right.clone());
        queue.push(t2.left.clone());
    }
    true
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
// tree depth_first_search breadth_first_search
#[test]
fn test2_101() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    assert_eq!(true, is_symmetric(root1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    assert_eq!(false, is_symmetric(root2));
}
