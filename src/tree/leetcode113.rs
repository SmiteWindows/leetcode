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
// Runtime: 0 ms
// Memory Usage: 2.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut tmp = Vec::new();
    postorder(root.as_deref(), sum, &mut res, &mut tmp);
    res
}

fn postorder(
    root: Option<&RefCell<TreeNode>>,
    sum: i32,
    res: &mut Vec<Vec<i32>>,
    tmp: &mut Vec<i32>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        tmp.push(node.val);
        if node.left.is_none() && node.right.is_none() {
            if node.val == sum {
                res.push(tmp.to_vec());
            }
        } else {
            postorder(node.left.as_deref(), sum - node.val, res, tmp);
            postorder(node.right.as_deref(), sum - node.val, res, tmp);
        }
        tmp.pop();
    }
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
