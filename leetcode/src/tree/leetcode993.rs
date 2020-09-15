// https://leetcode-cn.com/problems/cousins-in-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut rx = None;
    let mut ry = None;
    preorder(root.as_deref(), 0, 0, &mut rx, x);
    preorder(root.as_deref(), 0, 0, &mut ry, y);
    if let (Some((dx, px)), Some((dy, py))) = (rx, ry) {
        dx == dy && px != py
    } else {
        false
    }
}

fn preorder(
    root: Option<&RefCell<TreeNode>>,
    depth: usize,
    parent: i32,
    res: &mut Option<(usize, i32)>,
    v: i32,
) {
    if res.is_some() {
        return;
    }
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if v == val {
            *res = Some((depth, parent));
        }
        preorder(node.left.as_deref(), depth + 1, val, res, v);
        preorder(node.right.as_deref(), depth + 1, val, res, v);
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
fn test1_993() {
    use leetcode_prelude::btree;
    assert_eq!(is_cousins(btree![1, 2, 3, 4], 4, 3), false);

    assert_eq!(is_cousins(btree![1, 2, 3, null, 4, null, 5], 5, 4), true);

    assert_eq!(is_cousins(btree![1, 2, 3, null, 4], 2, 3), false);
}
