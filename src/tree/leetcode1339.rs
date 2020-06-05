// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/
// Runtime: 20 ms
// Memory Usage: 10.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum = 0;
    let mut best = 0;
    walk(root.as_deref(), &mut sum);
    helper(root.as_deref(), &mut sum, &mut best);
    let res = best as i64 * (sum as i64 - best as i64) % 1_000_000_007;
    res as i32
}

fn walk(root: Option<&RefCell<TreeNode>>, sum: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        *sum += node.val;
        walk(node.left.as_deref(), sum);
        walk(node.right.as_deref(), sum);
    }
}

fn helper(root: Option<&RefCell<TreeNode>>, sum: &mut i32, best: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let curr = node.val
            + helper(node.left.as_deref(), sum, best)
            + helper(node.right.as_deref(), sum, best);
        if (curr * 2 - *sum).abs() < (*best * 2 - *sum).abs() {
            *best = curr;
        }
        curr
    } else {
        0
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
// tree depth_first_search dynamic_programming
#[test]
fn test1_1339() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        }))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None,
    })));
    assert_eq!(max_product(t1), 110);
    assert_eq!(max_product(t2), 90);
    assert_eq!(max_product(t3), 1025);
    assert_eq!(max_product(t4), 1);
}
