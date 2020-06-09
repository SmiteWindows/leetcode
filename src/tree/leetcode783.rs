// https://leetcode.com/problems/minimum-distance-between-bst-nodes/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut prev = None;
    let mut min = i32::MAX;
    inorder(root.as_deref(), &mut prev, &mut min);
    min
}

fn inorder(root: Option<&RefCell<TreeNode>>, prev: &mut Option<i32>, min: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder(node.left.as_deref(), prev, min);
        let val = node.val;
        if let Some(prev_val) = prev.as_mut() {
            *min = (*min).min(val - *prev_val);
            *prev_val = val;
        } else {
            *prev = Some(val);
        }
        inorder(node.right.as_deref(), prev, min);
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
// tree recursion
#[test]
fn test1_783() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
    }))); // [4,2,6,1,3,null,null]
    assert_eq!(1, min_diff_in_bst(t1));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    })));
    assert_eq!(1, min_diff_in_bst(t2));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 27,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 34,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 58,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 50,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(44)))),
                    right: None,
                }))),
                right: None,
            }))),
        }))),
    })));
    assert_eq!(6, min_diff_in_bst(t3));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 90,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 69,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 49,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(52)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(89)))),
        }))),
        right: None,
    })));
    assert_eq!(1, min_diff_in_bst(t4));
}
