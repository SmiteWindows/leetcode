// https://leetcode.com/problems/minimum-depth-of-binary-tree/
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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = VecDeque::new();
    if root.is_none() {
        return 0;
    } else {
        stack.push_back((root, 1));
    }
    let mut current_depth = 0;
    while !stack.is_empty() {
        let (node, depth) = stack.pop_front().unwrap();
        current_depth = depth;
        let node = node.unwrap();
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            break;
        }
        if node.left.is_some() {
            stack.push_back((node.left.clone(), depth + 1));
        }
        if node.right.is_some() {
            stack.push_back((node.right.clone(), depth + 1));
        }
    }
    current_depth
}
// tree depth_first_search breadth_first_search
#[test]
fn test3_111() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(2, min_depth(root));
}
