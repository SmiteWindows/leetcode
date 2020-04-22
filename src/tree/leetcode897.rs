// https://leetcode.com/problems/increasing-order-search-tree/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        next: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let res = walk(left.as_ref(), Some(node.clone()));
            node.borrow_mut().right = walk(right.as_ref(), next);
            res
        } else {
            next
        }
    }
    
    walk(root.as_ref(), None)
}
// tree depth_first_search
#[test]
fn test1_897() {
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
