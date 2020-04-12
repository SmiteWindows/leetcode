// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
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
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, min: i32) -> i32 {
        if let Some(node) = root {
            if node.as_ref().borrow().val > min {
                return node.as_ref().borrow().val;
            }
            let left = walk(node.as_ref().borrow().left.as_ref(), min);
            let right = walk(node.as_ref().borrow().right.as_ref(), min);
            if left == -1 {
                return right;
            }
            if right == -1 {
                return left;
            }
            std::cmp::min(left, right)
        } else {
            -1
        }
    }
    if let Some(node)=root{
        walk(Some(&node), node.as_ref().borrow().val)
    }else{
        -1
    }
}
// tree
#[test]
fn test1_671() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(find_second_minimum_value(t1), 5);
    assert_eq!(find_second_minimum_value(t2), -1);
}
