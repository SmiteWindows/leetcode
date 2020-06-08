// https://leetcode.com/problems/univalued-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut val = None;
    preorder(root.as_deref(), &mut val)
}

fn preorder(root: Option<&RefCell<TreeNode>>, val: &mut Option<i32>) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        let node_val = node.val;
        if let Some(val) = val {
            if *val != node_val {
                return false;
            }
        } else {
            *val = Some(node_val);
        }
        preorder(node.left.as_deref(), val) && preorder(node.right.as_deref(), val)
    } else {
        true
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
fn test1_965() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    }))); // [1,1,1,1,1,null,1]
    assert_eq!(is_unival_tree(t1), true);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    }))); // [2,2,2,5,2]
    assert_eq!(is_unival_tree(t2), false);
}
