// https://leetcode.com/problems/binary-tree-right-side-view/
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
use std::{cell::RefCell, rc::Rc, collections::VecDeque};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res= Vec::new();
    if root.is_none() {
        return res;
    }
    let mut queue =VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let len=queue.len();
        for i in 0..len {
            let node = queue.pop_front().unwrap().unwrap();
            let node=node.borrow();
            if node.left.is_some(){
                queue.push_back(node.left.clone());
            }
            if node.right.is_some(){
                queue.push_back(node.right.clone());
            }
            if i==len-1{
                res.push(node.val);
            }
        }
    }
    res
}
// tree depth_first_search breadth_first_search
#[test]
fn test2_199() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    })));
    let res = vec![1, 3, 4];
    assert_eq!(res, right_side_view(root));
}
