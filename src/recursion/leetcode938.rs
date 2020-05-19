// https://leetcode.com/problems/range-sum-of-bst/
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
// Runtime: 8 ms
// Memory Usage: 4 MB
use std::{cell::RefCell, rc::Rc};
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, l: i32, r: i32, mut res: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let node_val = node.val;
            if l <= node_val && r >= node_val {
                res += node_val;
            }
            if l < node_val {
                res = walk(node.left.as_ref(), l, r, res);
            }
            if r > node_val {
                res = walk(node.right.as_ref(), l, r, res);
            }
        }
        res
    }

    walk(root.as_ref(), l, r, 0)
}
// tree recursion
#[test]
fn test2_938() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(18)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(18)))),
        }))),
    })));
    assert_eq!(range_sum_bst(t1, 7, 15), 32);
    assert_eq!(range_sum_bst(t2, 6, 10), 23);
}
