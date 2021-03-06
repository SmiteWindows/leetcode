// https://leetcode-cn.com/problems/delete-leaves-with-a-given-value/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::{cell::RefCell, rc::Rc};
pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(root, target)
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let val = node.borrow().val;
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        let left = postorder(left, target);
        let right = postorder(right, target);
        if left.is_none() && right.is_none() && val == target {
            None
        } else {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    } else {
        None
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
fn test1_1325() {
    use leetcode_prelude::btree;
    assert_eq!(
        remove_leaf_nodes(btree![1, 2, 3, 2, null, 2, 4], 2),
        btree![1, null, 3, null, 4]
    );

    assert_eq!(
        remove_leaf_nodes(btree![1, 3, 3, 3, 2], 3),
        btree![1, 3, null, null, 2]
    );

    assert_eq!(
        remove_leaf_nodes(btree![1, 2, null, 2, null, 2], 2),
        btree![1]
    );

    assert_eq!(remove_leaf_nodes(btree![1, 1, 1], 1), btree![]);

    assert_eq!(remove_leaf_nodes(btree![1, 2, 3], 1), btree![1, 2, 3]);
}
