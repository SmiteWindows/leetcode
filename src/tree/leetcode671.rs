// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
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
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root.as_deref() {
        walk(Some(node), node.borrow().val)
    } else {
        -1
    }
}

fn walk(root: Option<&RefCell<TreeNode>>, min: i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        if node.val > min {
            return node.val;
        }
        let left = walk(node.left.as_deref(), min);
        let right = walk(node.right.as_deref(), min);
        if left == -1 {
            return right;
        }
        if right == -1 {
            return left;
        }
        left.min(right)
    } else {
        -1
    }
}
// tree
#[test]
fn test1_671() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(find_second_minimum_value(t1), 5);
    assert_eq!(find_second_minimum_value(t2), -1);
}
