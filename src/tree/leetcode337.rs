// https://leetcode.com/problems/house-robber-iii/
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
// Memory Usage: 2.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![0; 2];
        if let Some(node) = root {
            let node = node.borrow();
            let left = walk(node.left.as_ref());
            let right = walk(node.right.as_ref());
            res[0] = left.iter().max().unwrap() + right.iter().max().unwrap();
            res[1] = left[0] + right[0] + node.val;
        }
        res
    }

    let res = walk(root.as_ref());
    *res.iter().max().unwrap()
}
// tree depth_first_search
#[test]
fn test1_337() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    })));
    assert_eq!(7, rob(t1));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    })));
    assert_eq!(9, rob(t2));
}
