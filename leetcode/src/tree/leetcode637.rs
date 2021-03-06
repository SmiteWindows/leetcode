// https://leetcode-cn.com/problems/average-of-levels-in-binary-tree/
// Runtime: 0 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, rc::Rc};
pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut count = Vec::new();
    let mut res = Vec::new();
    average(root.as_deref(), 0, &mut res, &mut count);
    let len = res.len();
    for i in 0..len {
        res[i] /= count[i] as f64;
    }
    res
}

fn average(root: Option<&RefCell<TreeNode>>, i: usize, sum: &mut Vec<f64>, count: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        if i < sum.len() {
            sum[i] += node.val as f64;
            count[i] += 1;
        } else {
            sum.push(node.val.into());
            count.push(1);
        }
        average(node.left.as_deref(), i + 1, sum, count);
        average(node.right.as_deref(), i + 1, sum, count);
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
// tree
#[test]
fn test1_637() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res: Vec<f64> = vec![3.0, 14.5, 11.0];
    assert_eq!(res, average_of_levels(root));
}
