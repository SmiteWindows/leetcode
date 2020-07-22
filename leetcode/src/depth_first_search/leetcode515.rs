// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
// Runtime: 0 ms
// Memory Usage: 2.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    walk(root.as_deref(), &mut res, 0);
    res
}

fn walk(root: Option<&RefCell<TreeNode>>, res: &mut Vec<i32>, level: usize) {
    if let Some(node) = root {
        let node = node.borrow();
        if res.len() == level {
            res.push(i32::MIN);
        }
        res[level] = node.val.max(res[level]);
        if node.left.is_some() {
            walk(node.left.as_deref(), res, level + 1);
        }
        if node.right.is_some() {
            walk(node.right.as_deref(), res, level + 1);
        }
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
// tree depth_first_search breadth_first_search
#[test]
fn test3_515() {
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
