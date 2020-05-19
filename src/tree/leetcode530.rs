// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
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
// Runtime: 0 ms
// Memory Usage: 2.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, mut res: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            res = walk(node.left.as_ref(), prev, res);
            let val = node.val;
            if let Some(n) = prev {
                res = std::cmp::min(res, val - *n);
            }
            *prev = Some(val);
            res = walk(node.right.as_ref(), prev, res);
        }
        res
    }

    let (mut prev, res) = (None, i32::MAX);
    walk(root.as_ref(), &mut prev, res)
}
// tree
#[test]
fn test1_530() {
    let t0 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        }))),
    })));
    assert_eq!(1, get_minimum_difference(t0));
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
    })));
    assert_eq!(1, get_minimum_difference(t1));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    })));
    assert_eq!(1, get_minimum_difference(t2));
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
    assert_eq!(6, get_minimum_difference(t3));
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
    assert_eq!(1, get_minimum_difference(t4));
}
