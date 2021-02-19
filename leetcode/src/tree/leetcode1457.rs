// https://leetcode-cn.com/problems/pseudo-palindromic-paths-in-a-binary-tree/
// Runtime: 28 ms
// Memory Usage: 11.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), 0, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, mut path: u32, all: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        path ^= 1 << val;
        if node.left.is_none() && node.right.is_none() && path.count_ones() < 2 {
            *all += 1;
        }
        preorder(node.left.as_deref(), path, all);
        preorder(node.right.as_deref(), path, all);
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
// tree depth_first_search bit_manipulation
#[test]
fn test1_1457() {
    use leetcode_prelude::btree;
    assert_eq!(pseudo_palindromic_paths(btree![2, 3, 1, 3, 1, null, 1]), 2);

    assert_eq!(
        pseudo_palindromic_paths(btree![2, 1, 1, 1, 3, null, null, null, null, null, 1]),
        1
    );

    assert_eq!(pseudo_palindromic_paths(btree![9]), 1);
}
