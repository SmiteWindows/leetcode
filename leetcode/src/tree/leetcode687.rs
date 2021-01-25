// https://leetcode-cn.com/problems/longest-univalue-path/
// Runtime: 20 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, rc::Rc};
pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = 0;
    lup(root.as_deref(), &mut max, 0);
    max
}

fn lup(root: Option<&RefCell<TreeNode>>, max: &mut i32, parent_val: i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = lup(node.left.as_deref(), max, val);
        let right = lup(node.right.as_deref(), max, val);
        *max = (*max).max(left + right);
        if parent_val == val {
            left.max(right) + 1
        } else {
            0
        }
    } else {
        0
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
// tree recursion
#[test]
fn test1_687() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));
    assert_eq!(2, longest_univalue_path(t1));
    assert_eq!(2, longest_univalue_path(t2));
}
