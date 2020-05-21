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
// Runtime: 4 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, cmp::Ordering, rc::Rc};
pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_depth = 0;
    let mut total = 0;
    walk(root.as_ref(), 0, &mut max_depth, &mut total);
    total
}

fn walk(root: Option<&Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, total: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        match depth.cmp(max_depth) {
            Ordering::Equal => *total += node.val,
            Ordering::Greater => {
                *max_depth = depth;
                *total = node.val;
            }
            Ordering::Less => {}
        }
        walk(node.left.as_ref(), depth + 1, max_depth, total);
        walk(node.right.as_ref(), depth + 1, max_depth, total);
    }
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
