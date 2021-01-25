// https://leetcode-cn.com/problems/binary-tree-right-side-view/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    preorder(root.as_deref(), 0, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, level: usize, res: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if res.len() == level {
            res.push(val);
        } else {
            res[level] = val;
        }
        preorder(node.left.as_deref(), level + 1, res);
        preorder(node.right.as_deref(), level + 1, res);
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
fn test3_199() {
    use leetcode_prelude::btree;
    assert_eq!(
        right_side_view(btree![1, 2, 3, null, 5, null, 4]),
        vec![1, 3, 4]
    );
}
