// https://leetcode-cn.com/problems/distribute-coins-in-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    postorder(root.as_deref(), &mut res);
    res
}

fn postorder(root: Option<&RefCell<TreeNode>>, sum: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = postorder(node.left.as_deref(), sum);
        let right = postorder(node.right.as_deref(), sum);
        let m = val + left + right - 1;
        *sum += m.abs();
        m
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
// tree depth_first_search
#[test]
fn test2_979() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(distribute_coins(t1), 2);
    assert_eq!(distribute_coins(t2), 3);
    assert_eq!(distribute_coins(t3), 2);
    assert_eq!(distribute_coins(t4), 4);
}
