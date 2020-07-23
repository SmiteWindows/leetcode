// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
// Runtime: 0 ms
// Memory Usage: 2.8 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut list = Vec::new();
    if root.is_none() {
        return list;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let len = queue.len();
        let mut max_value = i32::MIN;
        for i in 0..len {
            let tmp = queue.pop_front().unwrap().unwrap();
            let tmp = tmp.borrow();
            max_value = tmp.val.max(max_value);
            if tmp.left.is_some() {
                queue.push_back(tmp.left.clone());
            }
            if tmp.right.is_some() {
                queue.push_back(tmp.right.clone());
            }
        }
        list.push(max_value);
    }
    list
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
// tree depth_first_search breadth_first_search
#[test]
fn test1_515() {
    let res = vec![1, 3, 9];
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));
    assert_eq!(res, largest_values(root));
}
