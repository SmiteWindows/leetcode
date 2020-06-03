// https://leetcode.com/problems/binary-tree-level-order-traversal/
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
// Memory Usage: 2.1 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
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
                let n = queue.pop_front().expect("exist").expect("exist");
                let n = n.borrow();
                res[level].push(n.val);
                if n.left.is_some() {
                    queue.push_back(n.left.clone());
                }
                if n.right.is_some() {
                    queue.push_back(n.right.clone());
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
