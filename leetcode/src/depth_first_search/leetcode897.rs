// https://leetcode-cn.com/problems/increasing-order-search-tree/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    inorder(root, None)
}

fn inorder(
    root: Option<Rc<RefCell<TreeNode>>>,
    next: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.as_deref() {
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        let res = inorder(left, root.clone());
        node.borrow_mut().right = inorder(right, next);
        res
    } else {
        next
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
fn test2_897() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
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
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 7,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 8,
                                    left: None,
                                    right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                                }))),
                            }))),
                        }))),
                    }))),
                }))),
            }))),
        }))),
    })));
    assert_eq!(increasing_bst(root), res);
}
