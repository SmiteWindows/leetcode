// https://leetcode.com/problems/find-bottom-left-tree-value/
// Runtime: 0 ms
// Memory Usage: 2.9 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut res = 0;
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap().unwrap();
        let cur = cur.borrow();
        if cur.right.is_some() {
            queue.push_back(cur.right.clone());
        }
        if cur.left.is_some() {
            queue.push_back(cur.left.clone());
        }
        res = cur.val;
    }
    res
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
fn test2_513() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    assert_eq!(1, find_bottom_left_value(root1));
    assert_eq!(7, find_bottom_left_value(root2));
}
