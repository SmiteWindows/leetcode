// https://leetcode.com/problems/binary-tree-maximum-path-sum/
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
use std::{cell::RefCell, cmp::max, rc::Rc};
// Runtime: 4 ms
// Memory Usage: 4.2 MB
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_gain(root: Option<&Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left_gain = max(max_gain(node.borrow().left.as_ref(), max_sum), 0);
            let right_gain = max(max_gain(node.borrow().right.as_ref(), max_sum), 0);
            *max_sum = max(*max_sum, val + left_gain + right_gain);
            val + max(left_gain, right_gain)
        } else {
            0
        }
    }
    let mut max_sum = std::i32::MIN;
    max_gain(root.as_ref(), &mut max_sum);
    max_sum
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
