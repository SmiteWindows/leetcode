// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/
// Runtime: 20 ms
// Memory Usage: 7.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    postorder(root.as_deref(), &mut res);
    res - 1
}

fn postorder(root: Option<&RefCell<TreeNode>>, max: &mut i32) -> (i32, i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let (_, left_right) = postorder(node.left.as_deref(), max);
        let (right_left, _) = postorder(node.right.as_deref(), max);
        let left = left_right + 1;
        let right = right_left + 1;
        *max = (*max).max(left);
        *max = (*max).max(right);
        (left, right)
    } else {
        (0, 0)
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

// tree dynamic_programming
#[test]
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
    }))); // [1,null,1,1,1,null,null,1,1,null,1,null,null,null,1,null,1]
    assert_eq!(longest_zig_zag(t1), 3);
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
    }))); // [1,1,1,null,1,null,null,1,1,null,1]
    assert_eq!(longest_zig_zag(t2), 4);
    let t3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(longest_zig_zag(t3), 0);
}
