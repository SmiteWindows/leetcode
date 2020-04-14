// https://leetcode.com/problems/path-sum-iii/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
// Runtime: 24 ms
// Memory Usage: 2.2 MB
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(node) = root {
            let mut res = 0;
            if node.borrow().val == sum {
                res += 1;
            }
            res += helper(node.borrow().right.as_ref(), sum - node.borrow().val);
            res += helper(node.borrow().left.as_ref(), sum - node.borrow().val);
            res
        } else {
            0
        }
    }
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(node) = root {
            let mut res = helper(Some(node), sum);
            res += walk(node.borrow().right.as_ref(), sum);
            res += walk(node.borrow().left.as_ref(), sum);
            res
        } else {
            0
        }
    }
    walk(root.as_ref(), sum)
}
// tree
#[test]
fn test1_437() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
        }))),
    })));
    assert_eq!(3, path_sum(root, 8));
}
