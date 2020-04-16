// https://leetcode.com/problems/binary-tree-level-order-traversal/
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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(Some(node));
        let mut level = 0;
        while !queue.is_empty() {
            res.push(Vec::new());
            let level_length = queue.len();
            for i in 0..level_length {
                let n = queue.pop_front().unwrap().unwrap();
                res[level].push(n.borrow().val);
                if n.borrow().left.as_ref().is_some() {
                    queue.push_back(n.borrow().left.clone());
                }
                if n.borrow().right.as_ref().is_some() {
                    queue.push_back(n.borrow().right.clone());
                }
            }
            level += 1;
        }
    }
    res
}
// tree breadth_first_search
#[test]
fn test2_102() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], level_order(root));
}
