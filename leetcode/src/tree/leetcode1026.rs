// https://leetcode-cn.com/problems/maximum-difference-between-node-and-ancestor/
// Runtime: 0 ms
// Memory Usage: 2.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), None, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, mut parent: Option<(i32, i32)>, abs: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        parent = if let Some((min, max)) = parent {
            *abs = (*abs).max((min - val).abs().max((max - val).abs()));
            Some((min.min(val), max.max(val)))
        } else {
            Some((val, val))
        };
        preorder(node.left.as_deref(), parent, abs);
        preorder(node.right.as_deref(), parent, abs);
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
fn test1_1026() {
    use leetcode_prelude::btree;
    assert_eq!(
        max_ancestor_diff(btree![8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13]),
        7
    );
}
