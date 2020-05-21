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
// Runtime: 28 ms
// Memory Usage: 10.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = 0;
    walk(root.as_ref(), &mut max_sum);
    max_sum
}

fn walk(root: Option<&Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> (bool, i32, i32, i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_ref(), max_sum);
        let right = walk(node.right.as_ref(), max_sum);
        let mut sum = 0;
        if !left.0 || !right.0 || node.val >= right.1 || node.val <= left.2 {
            return (false, 0, 0, 0);
        }
        let cur_min = if node.left.is_some() {
            left.1
        } else {
            node.val
        };
        let cur_max = if node.right.is_some() {
            right.2
        } else {
            node.val
        };
        sum += node.val + left.3 + right.3;
        *max_sum = sum.max(*max_sum);
        (true, cur_min, cur_max, sum)
    } else {
        (true, i32::MAX, i32::MIN, 0)
    }
}
// binary_search_tree dynamic_programming
#[test]
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
