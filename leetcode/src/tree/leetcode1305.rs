// https://leetcode-cn.com/problems/all-elements-in-two-binary-search-trees/
// Runtime: 32 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, rc::Rc};
pub fn get_all_elements(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut res = vec![];
    inorder(root1.as_deref(), &mut res);
    inorder(root2.as_deref(), &mut res);
    res.sort_unstable();
    res
}

fn inorder(root: Option<&RefCell<TreeNode>>, v: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        inorder(node.left.as_deref(), v);
        v.push(val);
        inorder(node.right.as_deref(), v);
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
// tree sort
#[test]
fn test1_1305() {
    use leetcode_prelude::btree;
    assert_eq!(
        get_all_elements(btree![2, 1, 4], btree![1, 0, 3]),
        vec![0, 1, 1, 2, 3, 4]
    );

    assert_eq!(
        get_all_elements(btree![0, -10, 10], btree![5, 1, 7, 0, 2]),
        vec![-10, 0, 0, 1, 2, 5, 7, 10]
    );

    assert_eq!(
        get_all_elements(btree![], btree![5, 1, 7, 0, 2]),
        vec![0, 1, 2, 5, 7]
    );

    assert_eq!(
        get_all_elements(btree![0, -10, 10], btree![]),
        vec![-10, 0, 10]
    );

    assert_eq!(
        get_all_elements(btree![1, null, 8], btree![8, 1]),
        vec![1, 1, 8, 8]
    );
}
