// https://leetcode.com/problems/deepest-leaves-sum/
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
// Memory Usage: 3 MB
pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        depth: i32,
        max_depth: &mut i32,
        total: &mut i32,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            if depth > *max_depth {
                *max_depth = depth;
                *total = node.val;
            } else if depth == *max_depth {
                *total += node.val;
            }
            walk(node.left.as_ref(), depth + 1, max_depth, total);
            walk(node.right.as_ref(), depth + 1, max_depth, total);
        }
    }

    let mut max_depth = 0;
    let mut total = 0;
    walk(root.as_ref(), 0, &mut max_depth, &mut total);
    total
}
// tree depth_first_search
#[test]
fn test1_1302() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
        }))),
    })));
    assert_eq!(deepest_leaves_sum(root), 15);
}
