// https://leetcode-cn.com/problems/binary-tree-level-order-traversal/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    helper(root.as_deref(), 0, &mut res);
    res
}

fn helper(root: Option<&RefCell<TreeNode>>, level: usize, res: &mut Vec<Vec<i32>>) {
    if let Some(node) = root {
        let node = node.borrow();
        if level == res.len() {
            res.push(Vec::new());
        }
        res[level].push(node.val);
        if node.left.is_some() {
            helper(node.left.as_deref(), level + 1, res);
        }
        if node.right.is_some() {
            helper(node.right.as_deref(), level + 1, res);
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
// tree breadth_first_search
#[test]
fn test1_102() {
    use leetcode_prelude::{btree, vec2};
    assert_eq!(
        level_order(btree![3, 9, 20, null, null, 15, 7]),
        vec2![[3], [9, 20], [15, 7]]
    );
}
