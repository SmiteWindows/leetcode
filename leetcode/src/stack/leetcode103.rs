// https://leetcode-cn.com/problems/binary-tree-zigzag-level-order-traversal/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut levels = vec![];
    preorder(root.as_deref(), 0, &mut levels);
    for (i, level) in levels.iter_mut().enumerate() {
        if i % 2 == 1 {
            level.reverse();
        }
    }
    levels
}

fn preorder(root: Option<&RefCell<TreeNode>>, level: usize, levels: &mut Vec<Vec<i32>>) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if level == levels.len() {
            levels.push(vec![val]);
        } else {
            levels[level].push(val);
        }
        preorder(node.left.as_deref(), level + 1, levels);
        preorder(node.right.as_deref(), level + 1, levels);
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
fn test3_103() {
    use leetcode_prelude::{btree, vec2};
    assert_eq!(
        zigzag_level_order(btree![3, 9, 20, null, null, 15, 7]),
        vec2![[3], [20, 9], [15, 7]]
    );
}
