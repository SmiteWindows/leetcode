// https://leetcode.com/problems/leaf-similar-trees/
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
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut leaves1 = Vec::new();
    let mut leaves2 = Vec::new();
    walk(root1.as_ref(), &mut leaves1);
    walk(root2.as_ref(), &mut leaves2);
    leaves1 == leaves2
}

fn walk(root: Option<&Rc<RefCell<TreeNode>>>, leaf_values: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            leaf_values.push(node.val);
        }
        walk(node.left.as_ref(), leaf_values);
        walk(node.right.as_ref(), leaf_values);
    }
}
// tree depth_first_search
#[test]
fn test1_872() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        }))),
    })));
    assert_eq!(leaf_similar(t1, t2), true);
}
