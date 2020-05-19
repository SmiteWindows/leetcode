// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
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
pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let mut tmp = Vec::new();
        let len = queue.len();
        for i in 0..len {
            let node = queue.pop_front().unwrap().unwrap();
            let node = node.borrow();
            tmp.push(node.val);
            if node.left.is_some() {
                queue.push_back(node.left.clone());
            }
            if node.right.is_some() {
                queue.push_back(node.right.clone());
            }
        }
        res.insert(0, tmp);
    }
    res
}
// tree breadth_first_search
#[test]
fn test2_107() {
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
