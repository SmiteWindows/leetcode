// https://leetcode-cn.com/problems/recover-binary-search-tree/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{cell::RefCell, mem::swap, rc::Rc};
pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut prev = None;
    let mut first = None;
    let mut second = None;
    inorder(root, &mut prev, &mut first, &mut second);
    swap(
        &mut first.unwrap().borrow_mut().val,
        &mut second.unwrap().borrow_mut().val,
    )
}

fn inorder(
    root: &Option<Rc<RefCell<TreeNode>>>,
    prev: &mut Option<Rc<RefCell<TreeNode>>>,
    first: &mut Option<Rc<RefCell<TreeNode>>>,
    second: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder(&node.left, prev, first, second);
        if let Some(prev_val) = prev.clone() {
            if prev_val.borrow().val >= node.val {
                if first.is_none() {
                    *first = prev.clone();
                }
                *second = root.clone();
            }
        }
        *prev = root.clone();
        inorder(&node.right, prev, first, second);
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
fn test1_99() {
    use leetcode_prelude::btree;
    let mut t1 = btree![1, 3, null, null, 2];
    let res1 = btree![3, 1, null, null, 2];
    recover_tree(&mut t1);
    assert_eq!(res1, t1);
    let mut t2 = btree![3, 1, 4, null, null, 2];
    let res2 = btree![2, 1, 4, null, null, 3];
    recover_tree(&mut t2);
    assert_eq!(res2, t2);
}
