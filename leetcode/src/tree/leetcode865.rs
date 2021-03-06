// https://leetcode-cn.com/problems/smallest-subtree-with-all-the-deepest-nodes/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{
    cell::RefCell,
    cmp::Ordering::{Equal, Greater, Less},
    rc::Rc,
};
pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(&root).1
}

fn postorder(root: &Option<Rc<RefCell<TreeNode>>>) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let node = node.borrow();
        let (left_depth, left_tree) = postorder(&node.left);
        let (right_depth, rigth_tree) = postorder(&node.right);
        match left_depth.cmp(&right_depth) {
            Equal => (left_depth + 1, root.clone()),
            Less => (right_depth + 1, rigth_tree),
            Greater => (left_depth + 1, left_tree),
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
// tree
#[test]
fn test1_865() {
    use leetcode_prelude::btree;
    assert_eq!(
        subtree_with_all_deepest(btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]),
        btree![2, 7, 4]
    );
}
