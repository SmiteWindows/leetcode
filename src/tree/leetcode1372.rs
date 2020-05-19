// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/
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
pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!()
}
// tree dynamic_programming
#[test]
#[ignore]
fn test1_1372() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(longest_zig_zag(t1), 3);
    assert_eq!(longest_zig_zag(t2), 4);
    assert_eq!(longest_zig_zag(t3), 0);
}
