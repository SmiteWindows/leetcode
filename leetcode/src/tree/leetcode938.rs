// https://leetcode.com/problems/range-sum-of-bst/
// Runtime: 16 ms
// Memory Usage: 4 MB
use std::{cell::RefCell, rc::Rc};
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    let mut sum = 0;
    preorder(root.as_deref(), l, r, &mut sum);
    sum
}

fn preorder(root: Option<&RefCell<TreeNode>>, l: i32, r: i32, sum: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if val >= l && val <= r {
            *sum += val;
        }
        if val > l {
            preorder(node.left.as_deref(), l, r, sum)
        }
        if val < r {
            preorder(node.right.as_deref(), l, r, sum);
        }
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
fn test1_938() {
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
    }))); // [10,5,15,3,7,null,18]
    assert_eq!(range_sum_bst(t1, 7, 15), 32);
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
    }))); // [10,5,15,3,7,13,18,1,null,6]
    assert_eq!(range_sum_bst(t2, 6, 10), 23);
}
