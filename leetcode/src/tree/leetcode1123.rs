// https://leetcode-cn.com/problems/lowest-common-ancestor-of-deepest-leaves/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{
    cell::RefCell,
    cmp::Ordering::{Equal, Greater, Less},
    rc::Rc,
};
pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(&root).1
}

fn postorder(root: &Option<Rc<RefCell<TreeNode>>>) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let (left, left_lca) = postorder(&node.borrow().left);
        let (right, right_lca) = postorder(&node.borrow().right);
        match left.cmp(&right) {
            Equal => (left + 1, root.clone()),
            Greater => (left + 1, left_lca),
            Less => (right + 1, right_lca),
        }
    } else {
        (0, None)
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
fn test1_1123() {
    use leetcode_prelude::btree;
    assert_eq!(lca_deepest_leaves(btree![1, 2, 3]), btree![1, 2, 3]);

    assert_eq!(lca_deepest_leaves(btree![1, 2, 3, 4]), btree![4]);

    assert_eq!(lca_deepest_leaves(btree![1, 2, 3, 4, 5]), btree![2, 4, 5]);
}
