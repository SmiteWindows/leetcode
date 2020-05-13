// https://leetcode.com/problems/binary-tree-cameras/
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

pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!()
}
// tree dynamic_programming depth_first_search
#[test]
#[ignore]
fn test2_968() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        }))),
        right: None,
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                }))),
                right: None,
            }))),
            right: None,
        }))),
        right: None,
    })));
    assert_eq!(min_camera_cover(t1), 1);
    assert_eq!(min_camera_cover(t2), 2);
}
