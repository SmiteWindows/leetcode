// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(Some(node));
        queue.push_back(None);
        let mut level_list = Vec::new();
        let mut is_order_left = true;
        while !queue.is_empty() {
            if let Some(curr) = queue.pop_front().unwrap() {
                let curr = curr.borrow();
                if is_order_left {
                    level_list.push(curr.val);
                } else {
                    level_list.insert(0, curr.val);
                }
                if curr.left.is_some() {
                    queue.push_back(curr.left.clone());
                }
                if curr.right.is_some() {
                    queue.push_back(curr.right.clone());
                }
            } else {
                res.push(level_list);
                level_list = Vec::new();
                if !queue.is_empty() {
                    queue.push_back(None);
                }
                is_order_left = !is_order_left;
            }
        }
    }
    res
}
// tree breadth_first_search stack
#[test]
fn test2_103() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(
        vec![vec![3], vec![20, 9], vec![15, 7]],
        zigzag_level_order(root)
    );
}
