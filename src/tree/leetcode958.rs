// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut flag = false;
    while !queue.is_empty() {
        for i in 0..queue.len() {
            let curr = queue.pop_front().expect("exist");
            if curr.is_some() {
                if flag {
                    return false;
                }
                let curr = curr.as_ref().expect("exist").borrow();
                queue.push_back(curr.left.clone());
                queue.push_back(curr.right.clone());
            } else {
                flag = true;
            }
        }
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
// tree
#[test]
fn test1_958() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(is_complete_tree(t1), true);
    assert_eq!(is_complete_tree(t2), false);
}
