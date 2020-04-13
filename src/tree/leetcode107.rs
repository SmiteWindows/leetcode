// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
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
// Memory Usage: 2.1 MB
pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            if level >= res.len() {
                let mut new_level = Vec::new();
                new_level.push(node.borrow().val);
                res.push(new_level);
            } else {
                res[level].push(node.borrow().val);
            }
            if node.borrow().left.as_ref().is_some() {
                helper(node.borrow().left.as_ref(), level + 1, res);
            }
            if node.borrow().right.as_ref().is_some() {
                helper(node.borrow().right.as_ref(), level + 1, res);
            }
        }
    }
    helper(root.as_ref(), 0, &mut res);
    res.reverse();
    res
}
// tree breadth_first_search
#[test]
fn test1_107() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res = vec![vec![15, 7], vec![9, 20], vec![3]];
    assert_eq!(res, level_order_bottom(root));
}
