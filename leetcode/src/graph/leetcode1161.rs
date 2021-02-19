// https://leetcode-cn.com/problems/maximum-level-sum-of-a-binary-tree/
// Runtime: 32 ms
// Memory Usage: 3.2 MB
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum = HashMap::new();
    preorder(root.as_deref(), 1, &mut sum);
    *sum.iter().max_by_key(|(_, &v)| v).unwrap().0 as i32
}

fn preorder(root: Option<&RefCell<TreeNode>>, level: usize, sum: &mut HashMap<usize, i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        *sum.entry(level).or_default() += node.val;
        preorder(node.left.as_deref(), level + 1, sum);
        preorder(node.right.as_deref(), level + 1, sum);
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
// graph
#[test]
fn test1_1161() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(max_level_sum(root), 2);
}
