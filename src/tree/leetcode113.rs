// https://leetcode.com/problems/path-sum-ii/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.5 MB
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut tmp = Vec::new();
    fn path(
        root: Option<&Rc<RefCell<TreeNode>>>,
        sum: i32,
        res: &mut Vec<Vec<i32>>,
        tmp: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            tmp.push(node.borrow().val);
            if node.borrow().left.is_none()
                && node.borrow().right.is_none()
                && sum == node.borrow().val
            {
                res.push(tmp.to_vec());
            }
            path(
                node.borrow().left.as_ref(),
                sum - node.borrow().val,
                res,
                tmp,
            );
            path(
                node.borrow().right.as_ref(),
                sum - node.borrow().val,
                res,
                tmp,
            );
            tmp.pop();
        }
    }
    path(root.as_ref(), sum, &mut res, &mut tmp);
    res
}
// tree depth_first_search
#[test]
fn test1_113() {
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
    let res = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
    assert_eq!(res, path_sum(root, 22));
}
