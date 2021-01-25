// https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/
// Runtime: 4 ms
// Memory Usage: 4.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = i32::MIN;
    postorder(root.as_deref(), &mut max_sum);
    max_sum
}

fn postorder(root: Option<&RefCell<TreeNode>>, max_sum: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left_gain = postorder(node.left.as_deref(), max_sum).max(0);
        let right_gain = postorder(node.right.as_deref(), max_sum).max(0);
        *max_sum = (val + left_gain + right_gain).max(*max_sum);
        val + left_gain.max(right_gain)
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
// tree depth_first_search
#[test]
fn test2_124() {
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
