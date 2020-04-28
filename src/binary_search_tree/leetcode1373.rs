// https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/
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

pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!()
}
// binary_search_tree dynamic_programming
#[test]
#[ignore]
fn test1_1373() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: None,
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: -4,
        left: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
    })));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    assert_eq!(20, max_sum_bst(t1));
    assert_eq!(2, max_sum_bst(t2));
    assert_eq!(0, max_sum_bst(t3));
    assert_eq!(6, max_sum_bst(t4));
    assert_eq!(7, max_sum_bst(t5));
}
