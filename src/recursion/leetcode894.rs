// https://leetcode.com/problems/all-possible-full-binary-trees/
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

pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    todo!()
}
// tree recursion
#[test]
#[ignore]
fn test1_894() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            }))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        }))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        }))),
    })));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    let t5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(all_possible_fbt(7), vec![t1, t2, t3, t4, t5]);
}
