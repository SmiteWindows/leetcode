// https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut prev = None;
    postorder(root.take(), &mut prev);
    *root = prev;
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        postorder(right, prev);
        postorder(left, prev);
        node.borrow_mut().right = prev.take();
        *prev = Some(node);
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
fn test2_114() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    }))),
                }))),
            }))),
        }))),
    })));
    flatten(&mut root);
    assert_eq!(res, root);
}
