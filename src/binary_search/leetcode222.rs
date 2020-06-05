// https://leetcode.com/problems/count-complete-tree-nodes/
// Runtime: 4 ms
// Memory Usage: 4.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let d = compute_depth(root.clone());
    if d == 0 {
        return 1;
    }
    let mut left = 1 as usize;
    let mut right = 2usize.pow(d) - 1;
    let mut pivot;
    while left <= right {
        pivot = left + (right - left) / 2;
        if exists(root.clone(), pivot, d) {
            left = pivot + 1;
        } else {
            right = pivot - 1;
        }
    }
    let res = 2usize.pow(d) - 1 + left;
    res as i32
}

fn compute_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
    let mut d = 0;
    while root.as_ref().expect("exist").borrow().left.is_some() {
        let left = root.as_ref().expect("exist").borrow().left.clone();
        root = left;
        d += 1;
    }
    d
}

fn exists(mut root: Option<Rc<RefCell<TreeNode>>>, idx: usize, d: u32) -> bool {
    let mut left = 0 as usize;
    let mut right = 2usize.pow(d) - 1;
    let mut pivot;
    for i in 0..d {
        pivot = left + (right - left) / 2;
        if let Some(node) = root {
            if idx <= pivot {
                root = node.borrow().left.clone();
                right = pivot;
            } else {
                root = node.borrow().right.clone();
                left = pivot + 1;
            }
        }
    }
    root.is_some()
}

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
// tree binary_search
#[test]
fn test2_222() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    })));
    assert_eq!(6, count_nodes(root));
}
