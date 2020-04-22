// https://leetcode.com/problems/find-bottom-left-tree-value/
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
// Runtime: 0 ms
// Memory Usage: 2.9 MB
pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, level: i32, max_level: &mut i32, res: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            if level > *max_level {
                *max_level = level;
                *res = node.val;
            }
            walk(node.left.as_ref(), level + 1, max_level, res);
            walk(node.right.as_ref(), level + 1, max_level, res);
        }
    }

    let mut res = 0;
    let mut max_level = -1;
    walk(root.as_ref(), 0, &mut max_level, &mut res);
    res
}
// tree depth_first_search breadth_first_search
#[test]
fn test3_513() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    assert_eq!(1, find_bottom_left_value(root1));
    assert_eq!(7, find_bottom_left_value(root2));
}
