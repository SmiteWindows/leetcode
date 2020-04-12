// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
}
// tree depth_first_search
#[test]
#[ignore]
fn test1_1028() {
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: None,
        }))),
    })));
    let res3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 401,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 349,
                left: Some(Rc::new(RefCell::new(TreeNode::new(90)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(88)))),
        }))),
        right: None,
    })));
    assert_eq!(
        recover_from_preorder(String::from("1-2--3--4-5--6--7")),
        res1
    );
    assert_eq!(
        recover_from_preorder(String::from("1-2--3---4-5--6---7")),
        res2
    );
    assert_eq!(
        recover_from_preorder(String::from("1-401--349---90--88")),
        res3
    );
}
