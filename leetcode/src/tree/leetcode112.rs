// https://leetcode-cn.com/problems/path-sum/
// Runtime: 0 ms
// Memory Usage: 2.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    helper(root.as_deref(), sum)
}

fn helper(root: Option<&RefCell<TreeNode>>, mut sum: i32) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        sum -= node.val;
        if node.left.is_none() && node.right.is_none() {
            return sum == 0;
        }
        helper(node.left.as_deref(), sum) || helper(node.right.as_deref(), sum)
    } else {
        false
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
fn test1_112() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
    })));
    assert_eq!(true, has_path_sum(root, 22));
}
