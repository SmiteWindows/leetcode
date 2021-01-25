// https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/
// Runtime: 4 ms
// Memory Usage: 4.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = i32::MIN;
    postorder(root.as_deref(), &mut res);
    res
}
fn postorder(root: Option<&RefCell<TreeNode>>, max: &mut i32) -> Option<i32> {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left_max = postorder(node.left.as_deref(), max);
        let right_max = postorder(node.right.as_deref(), max);
        match (left_max, right_max) {
            (Some(left_max), Some(right_max)) => {
                *max = (*max).max(val);
                *max = (*max).max(val + left_max);
                *max = (*max).max(val + right_max);
                *max = (*max).max(val + left_max + right_max);
                Some(val + 0.max(left_max.max(right_max)))
            }
            (Some(left_max), None) => {
                *max = (*max).max(val);
                *max = (*max).max(val + left_max);
                Some(val + 0.max(left_max))
            }
            (None, Some(right_max)) => {
                *max = (*max).max(val);
                *max = (*max).max(val + right_max);
                Some(val + 0.max(right_max))
            }
            (None, None) => {
                *max = (*max).max(val);
                Some(val)
            }
        }
    } else {
        None
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
// tree depth_first_search
#[test]
fn test1_124() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(6, max_path_sum(root1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: -10,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(42, max_path_sum(root2));
}
