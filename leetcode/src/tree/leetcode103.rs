// https://leetcode-cn.com/problems/binary-tree-zigzag-level-order-traversal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    helper(root.as_deref(), 0, &mut res);
    res
}

fn helper(root: Option<&RefCell<TreeNode>>, level: usize, res: &mut Vec<Vec<i32>>) {
    if let Some(node) = root {
        let node = node.borrow();
        if level >= res.len() {
            let mut new_level = Vec::new();
            new_level.push(node.val);
            res.push(new_level);
        } else if level % 2 == 0 {
            res[level].push(node.val);
        } else {
            res[level].insert(0, node.val);
        }

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
// tree breadth_first_search stack
#[test]
fn test1_103() {
    use leetcode_prelude::vec2;
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(zigzag_level_order(root), vec2![[3], [20, 9], [15, 7]]);
}
