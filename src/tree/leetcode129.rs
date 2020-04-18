// https://leetcode.com/problems/sum-root-to-leaf-numbers/
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
// Memory Usage: 1.9 MB
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, cur: i32, sum: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            if node.left.is_none() && node.right.is_none() {
                *sum += cur * 10 + val;
                return;
            }
            walk(node.left.as_ref(), cur * 10 + val, sum);
            walk(node.right.as_ref(), cur * 10 + val, sum);
        }
    }
    let mut sum = 0;
    walk(root.as_ref(), 0, &mut sum);
    sum
}
// tree depth_first_search
#[test]
fn test1_129() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(25, sum_numbers(root1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(1026, sum_numbers(root2));
}
