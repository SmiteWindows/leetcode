// https://leetcode-cn.com/problems/flip-equivalent-binary-trees/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn flip_equiv(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    flip_eq(root1.as_deref(), root2.as_deref())
}

fn flip_eq(root1: Option<&RefCell<TreeNode>>, root2: Option<&RefCell<TreeNode>>) -> bool {
    if let Some(node1) = root1 {
        if let Some(node2) = root2 {
            let node1 = node1.borrow();
            let node2 = node2.borrow();
            if node1.val != node2.val {
                return false;
            }
            flip_eq(node1.left.as_deref(), node2.left.as_deref())
                && flip_eq(node1.right.as_deref(), node2.right.as_deref())
                || flip_eq(node1.left.as_deref(), node2.right.as_deref())
                    && flip_eq(node1.right.as_deref(), node2.left.as_deref())
        } else {
            false
        }
    } else {
        root2.is_none()
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
fn test1_951() {
    use leetcode_prelude::btree;
    assert_eq!(
        flip_equiv(
            btree![1, 2, 3, 4, 5, 6, null, null, null, 7, 8],
            btree![1, 3, 2, null, 6, 4, 5, null, null, null, null, 8, 7]
        ),
        true
    );
}
