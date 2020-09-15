// https://leetcode-cn.com/problems/maximum-binary-tree-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn insert_into_max_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    insert(root, val)
}

fn insert(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        if node.borrow().val > val {
            let right = node.borrow_mut().right.take();
            node.borrow_mut().right = insert(right, val);
            Some(node)
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Some(node),
                right: None,
            })))
        }
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
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
fn test1_998() {
    use leetcode_prelude::btree;
    assert_eq!(
        insert_into_max_tree(btree![4, 1, 3, null, null, 2], 5),
        btree![5, 4, null, 1, 3, null, null, 2]
    );

    assert_eq!(
        insert_into_max_tree(btree![5, 2, 4, null, 1], 3),
        btree![5, 2, 4, null, 1, null, 3]
    );

    assert_eq!(
        insert_into_max_tree(btree![5, 2, 3, null, 1], 4),
        btree![5, 2, 4, null, 1, 3]
    );
}
