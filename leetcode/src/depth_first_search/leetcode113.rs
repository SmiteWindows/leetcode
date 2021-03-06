// https://leetcode-cn.com/problems/path-sum-ii/
// Runtime: 0 ms
// Memory Usage: 2.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut tmp = Vec::new();
    path(root.as_deref(), sum, &mut res, &mut tmp);
    res
}

fn path(root: Option<&RefCell<TreeNode>>, sum: i32, res: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        tmp.push(node.val);
        if node.left.is_none() && node.right.is_none() && sum == node.val {
            res.push(tmp.to_vec());
        }
        path(node.left.as_deref(), sum - node.val, res, tmp);
        path(node.right.as_deref(), sum - node.val, res, tmp);
        tmp.pop();
    }
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
// tree depth_first_search
#[test]
fn test2_113() {
    use leetcode_prelude::vec2;
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
    })));
    assert_eq!(path_sum(root, 22), vec2![[5, 4, 11, 2], [5, 8, 4, 5]]);
}
